//! Regression cases from microscope's Chromium profiles.

fn demangle(name: &str) -> Option<String> {
    cpp_demangle::Symbol::new(name).ok()?.demangle().ok()
}

/// C++20's manglings, as Chromium's build has them: constrained
/// template parameters (`Tk`, `Tp`), whose constraint's name later
/// substitutions count; requires clauses (`Q`); template parameters of
/// an enclosing level (`TL`); and generic lambdas (`UlTy`). Each part
/// expected is `llvm-cxxfilt`'s.
#[test]
fn cpp20_constraints_requires_clauses_and_generic_lambdas_demangle() {
    for (mangled, parts) in [
        (
            "_ZN15partition_alloc13PartitionRoot10FreeInlineILNS_8internal9FreeFlagsE0ETpTkNS_8FreeHintEJEEEvPvDpT0_",
            &["partition_alloc::PartitionRoot::FreeInline<"][..],
        ),
        (
            "_ZN15partition_alloc13PartitionRoot17FreeInUnknownRootILNS_8internal9FreeFlagsE6ETpTkNS_8FreeHintEJNS2_4base11StrongAliasINS_15FreeSizeHintTagEmEEEEEvPvDpT0_",
            &[
                "partition_alloc::PartitionRoot::FreeInUnknownRoot<",
                "partition_alloc::internal::base::StrongAlias<partition_alloc::FreeSizeHintTag, unsigned long>",
            ],
        ),
        (
            "_ZN4base12ObserverListIN16OmniboxEditModel8ObserverELb0ELNS_28ObserverListReentrancyPolicyE0ENS_8internal22CheckedObserverAdapterEE21NotifyAllowReentrancyIMS2_FvvEJEQsr3stdE9invocableITL0__PT_DpRKTL0_0_EEEvSB_DpRKT0_",
            &[
                "base::ObserverList<OmniboxEditModel::Observer, false",
                "::NotifyAllowReentrancy<void (OmniboxEditModel::Observer::*)()>",
            ],
        ),
        (
            "_ZNKSt4__Cr6ranges13__upper_boundclB9sqn240000ITkNS0_13forward_rangeERN4base4spanIKN5blink17OffsetMappingUnitELm18446744073709551615EPS7_EEjMS6_KFjvETkNS_26indirect_strict_weak_orderIPKT0_NS_16__projected_implIDTclL_ZNS0_5__cpo5beginEEclsr3stdE7declvalIRT_EEEET1_E6__typeEEENS0_4lessEEENS_7_IfImplIX14borrowed_rangeISJ_EEE7_SelectISL_NS0_8danglingEEEOSJ_RSF_T2_SM_",
            &[
                "std::__Cr::ranges::__upper_bound::operator()",
                "base::span<blink::OffsetMappingUnit const",
            ],
        ),
        (
            "_ZNKSt4__Cr6ranges7__equalclB9sqn240000ITkNS0_11input_rangeEN4base4spanIKhLm18446744073709551615EPS5_EETkNS0_11input_rangeES7_NS0_8equal_toENS_8identityES9_Q21indirectly_comparableIDTclL_ZNS0_5__cpo5beginEEclsr3stdE7declvalIRT_EEEEDTclL_ZNSA_5beginEEclsr3stdE7declvalIRT0_EEEET1_T2_T3_EEEbOSB_OSE_SH_SI_SJ_",
            &[
                "std::__Cr::ranges::__equal::operator()",
                "std::__Cr::ranges::equal_to, std::__Cr::identity, std::__Cr::identity>",
            ],
        ),
        (
            "_ZNSt4__Cr24__copy_move_unwrap_itersB9sqn240000INS_11__copy_implEN4base25CheckedContiguousIteratorIKcEES5_NS3_IcEETnNS_9enable_ifIXsr12__can_rewrapIT0_T2_EE5valueEiE4typeELi0EEENS_15__in_out_resultIS8_S9_EES8_T1_S9_",
            &[
                "std::__Cr::__copy_move_unwrap_iters",
                "base::CheckedContiguousIterator<char>",
            ],
        ),
        (
            "_ZN5blink12_GLOBAL__N_124DetermineAlgorithmAndRunIZNS0_19LayoutWithAlgorithmERKNS_21LayoutAlgorithmParamsEEUlTyPT_E_EEvS4_RKS5_",
            &[
                "blink::(anonymous namespace)::DetermineAlgorithmAndRun<",
                "blink::(anonymous namespace)::LayoutWithAlgorithm(blink::LayoutAlgorithmParams const&)",
            ],
        ),
    ] {
        let demangled = demangle(mangled);
        for part in parts {
            assert!(
                demangled.as_deref().is_some_and(|name| name.contains(part)),
                "{}: {:?} lacks {}", mangled, demangled, part
            );
        }
    }
}

#[test]
fn empty_packs_do_not_leave_commas() {
    for (mangled, expected) in [
        ("_Z1fIJEEvDpT_", "void f<>()"),
        ("_Z1fIiJEEvT_DpT0_", "void f<int>(int)"),
    ] {
        assert_eq!(demangle(mangled).as_deref(), Some(expected), "{}", mangled);
    }
}
