use heck::SnakeCase;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{
    subst::GenericArgKind, AssocKind, GenericParamDefKind, TraitPredicate, TyCtxt,
};
use why3::{
    declaration::{CloneSubst, Contract, Decl, DeclClone, Module, TyDecl, Val},
    mlcfg::{QName, Type},
};

use super::TranslationCtx;

impl TranslationCtx<'_, '_> {
    pub fn trait_clones(&mut self, def_id: DefId) -> Vec<why3::declaration::Decl> {
        let traits = traits_used_by(self.tcx, def_id);

        let mut trait_clones = Vec::new();
        for t in traits {
            self.translate_trait(t.def_id());
            // let trait_def = self.tcx.explicit_predicates_of(t.def_id());
            let trait_params = self.tcx.generics_of(t.def_id());

            let params = trait_params.params.iter().zip(t.trait_ref.substs.into_iter());
            let mut subst = vec![];
            for (p, ty) in params {
                if let GenericParamDefKind::Type { .. } = p.kind {
                    let ty = super::ty::translate_ty(self, rustc_span::DUMMY_SP, ty.expect_ty());
                    subst.push(CloneSubst::Type(p.name.to_string().to_snake_case().into(), ty));
                }
            }

            let clone =
                Decl::Clone(DeclClone { name: translate_trait_name(self.tcx, t.def_id()), subst });

            trait_clones.push(clone);
        }
        trait_clones
    }

    pub fn translate_trait(&mut self, def_id: DefId) {
        if self.used_traits.contains(&def_id) {
            return;
        } else {
            self.used_traits.insert(def_id);
        }
        let params = self.tcx.generics_of(def_id);
        let mut trait_name = translate_trait_name(self.tcx, def_id);

        let mut param_tys = Vec::new();
        for p in &params.params {
            match p.kind {
                GenericParamDefKind::Type { .. } => {
                    let ty_name = QName {
                        module: trait_name.module.clone(),
                        name: vec![p.name.to_string().to_snake_case()],
                    };
                    param_tys.push(Decl::TyDecl(TyDecl {
                        ty_name,
                        ty_constructors: vec![],
                        ty_params: vec![],
                    }))
                }
                _ => {}
            }
        }

        // TODO: Clone super traits!
        param_tys.append(&mut self.trait_clones(def_id));

        for item in self.tcx.associated_items(def_id).in_definition_order() {
            match item.kind {
                AssocKind::Fn => {
                    let fn_sig = self.tcx.fn_sig(item.def_id).skip_binder();

                    let inputs = self
                        .tcx
                        .fn_arg_names(item.def_id)
                        .iter()
                        .zip(fn_sig.inputs().iter())
                        .map(|(name, ty)| {
                            (
                                name.to_string().into(),
                                super::ty::translate_ty(self, rustc_span::DUMMY_SP, ty),
                            )
                        })
                        .collect();
                    let output =
                        super::ty::translate_ty(self, rustc_span::DUMMY_SP, fn_sig.output());

                    let name = super::translate_value_id(self.tcx, item.def_id);
                    param_tys.push(Decl::ValDecl(Val {
                        name,
                        contract: Contract::new(),
                        params: inputs,
                        retty: output,
                    }))
                }
                AssocKind::Type => unimplemented!("associated type"),
                AssocKind::Const => unimplemented!("constants"),
            }
        }

        let trait_mod = Module { name: trait_name.name(), decls: param_tys };
        self.modules.add_decl(trait_name, Decl::Module(trait_mod));
    }
}

fn translate_trait_name(tcx: TyCtxt<'_>, def_id: DefId) -> QName {
    super::translate_value_id(tcx, def_id)
}

fn traits_used_by<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Vec<TraitPredicate<'tcx>> {
    let predicates = tcx.explicit_predicates_of(def_id);
    let mut traits = Vec::new();

    for (pred, _) in predicates.predicates {
        let inner = pred.kind().no_bound_vars().unwrap();
        use rustc_middle::ty::PredicateKind::*;

        match inner {
            Trait(tp, _) => traits.push(tp),
            _ => {}
        }
    }
    traits
}
