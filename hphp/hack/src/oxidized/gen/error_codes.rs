// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
//
// @generated SignedSource<<bf33d505bdb8e6c0c3ee6057fe959a11>>
//
// To regenerate this file, run:
//   hphp/hack/src/oxidized/regen.sh

use arena_trait::TrivialDrop;
use ocamlrep_derive::FromOcamlRep;
use ocamlrep_derive::FromOcamlRepIn;
use ocamlrep_derive::ToOcamlRep;
use serde::Deserialize;
use serde::Serialize;

#[allow(unused_imports)]
use crate::*;

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    FromOcamlRep,
    FromOcamlRepIn,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub enum Parsing {
    FixmeFormat = 1001,
    ParsingError,
    UnexpectedEofDEPRECATED,
    UnterminatedCommentDEPRECATED,
    UnterminatedXhpCommentDEPRECATED,
    CallTimePassByReferenceDEPRECATED,
    XhpParsingError,
}
impl TrivialDrop for Parsing {}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    FromOcamlRep,
    FromOcamlRepIn,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub enum Naming {
    AddATypehint = 2001,
    TypeparamAlokDEPRECATED,
    AssertArity,
    PrimitiveInvalidAlias,
    CyclicConstraintDEPRECATED,
    DidYouMeanNaming,
    DifferentScopeDEPRECATED,
    DisallowedXhpType,
    DoubleInsteadOfFloatDEPRECATED,
    DynamicClassDEPRECATED,
    LvarInObjGet,
    ErrorNameAlreadyBound,
    ExpectedCollection,
    ExpectedVariable,
    FdNameAlreadyBound,
    GenArrayRecArityDEPRECATED,
    GenArrayVaRecArityDEPRECATED,
    GenaArityDEPRECATED,
    GenericClassVarDEPRECATED,
    GenvaArityDEPRECATED,
    IllegalClass,
    IllegalClassMeth,
    IllegalConstant,
    IllegalFun,
    IllegalInstMeth,
    IllegalMethCaller,
    IllegalMethFun,
    IntegerInsteadOfIntDEPRECATED,
    InvalidReqExtends,
    InvalidReqImplements,
    LocalConstDEPRECATED,
    LowercaseThis,
    MethodNameAlreadyBound,
    MissingArrow,
    MissingTypehint,
    NameAlreadyBound,
    NamingTooFewArguments,
    NamingTooManyArguments,
    PrimitiveToplevel,
    RealInsteadOfFloatDEPRECATED,
    ShadowedTypeParam,
    StartWithT,
    ThisMustBeReturn,
    ThisNoArgument,
    ThisHintOutsideClass,
    ThisReserved,
    HigherKindedTypesUnsupportedFeature,
    TypedefConstraintDEPRECATED,
    UnboundName,
    Undefined,
    UnexpectedArrow,
    UnexpectedTypedef,
    UsingInternalClass,
    VoidCast,
    ObjectCast,
    UnsetCast,
    NullsafePropertyAccessDEPRECATED,
    IllegalTrait,
    ShapeTypehintDEPRECATED,
    DynamicNewInStrictMode,
    InvalidTypeAccessRoot,
    DuplicateUserAttribute,
    ReturnOnlyTypehint,
    UnexpectedTypeArguments,
    TooManyTypeArguments,
    ClassnameParam,
    InvalidInstanceofDEPRECATED,
    NameIsReserved,
    DollardollarUnused,
    IllegalMemberVariableClass,
    TooFewTypeArguments,
    GotoLabelAlreadyDefined,
    GotoLabelUndefined,
    GotoLabelDefinedInFinally,
    GotoInvokedInFinally,
    DynamicClassPropertyNameInStrictModeDEPRECATED,
    ThisAsLexicalVariable,
    DynamicClassNameInStrictMode,
    XhpOptionalRequiredAttr,
    XhpRequiredWithDefault,
    VariableVariablesDisallowedDEPRECATED,
    ArrayTypehintsDisallowed,
    ArrayLiteralsDisallowedDEPRECATED,
    WildcardDisallowed,
    AttributeClassNameConflict,
    MethodNeedsVisibility,
    ReferenceInStrictModeDEPRECATED,
    ReferenceInRxDEPRECATED,
    DeclareStatementDEPRECATED,
    MisplacedRxOfScopeDEPRECATED,
    RxOfScopeAndExplicitRxDEPRECATED,
    UnsupportedFeatureDEPRECATED,
    TraitInterfaceConstructorPromoDEPRECATED,
    NonstaticPropertyWithLSB,
    ReferenceInAnonUseClauseDEPRECATED,
    RxMoveInvalidLocation,
    MisplacedMutabilityHint,
    MutabilityHintInNonRx,
    InvalidReturnMutableHint,
    NoTparamsOnTypeConstsDEPRECATED,
    PocketUniversesDuplication,
    UnsupportedTraitUseAs,
    UnsupportedInsteadOf,
    InvalidTraitUseAsVisibility,
    InvalidFunPointer,
    IllegalUseOfDynamicallyCallable,
    PocketUniversesNotInClass,
    PocketUniversesAtomMissing,
    PocketUniversesAtomUnknown,
    PocketUniversesLocalization,
    ClassMethNonFinalSelf,
    ParentInFunctionPointer,
    SelfInNonFinalFunctionPointer,
}
impl TrivialDrop for Naming {}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    FromOcamlRep,
    FromOcamlRepIn,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub enum NastCheck {
    AbstractBody = 3001,
    AbstractWithBody,
    AwaitInSyncFunction,
    CallBeforeInit,
    CaseFallthrough,
    ContinueInSwitch,
    DangerousMethodNameDEPRECATED,
    DefaultFallthrough,
    InterfaceWithMemberVariable,
    InterfaceWithStaticMemberVariable,
    Magic,
    NoConstructParent,
    NonInterface,
    NotAbstractWithoutBody,
    NotInitialized,
    NotPublicInterface,
    RequiresNonClass,
    ReturnInFinally,
    ReturnInGen,
    ToStringReturnsString,
    ToStringVisibility,
    ToplevelBreak,
    ToplevelContinue,
    UsesNonTrait,
    IllegalFunctionName,
    NotAbstractWithoutTypeconst,
    TypeconstDependsOnExternalTparam,
    TypeconstAssignedTparamDEPRECATED,
    AbstractWithTypeconstDEPRECATED,
    ConstructorRequired,
    InterfaceWithPartialTypeconst,
    MultipleXhpCategory,
    OptionalShapeFieldsNotSupportedDEPRECATED,
    AwaitNotAllowedDEPRECATED,
    AsyncInInterfaceDEPRECATED,
    AwaitInCoroutine,
    YieldInCoroutine,
    SuspendOutsideOfCoroutine,
    SuspendInFinally,
    BreakContinueNNotSupportedDEPRECATED,
    StaticMemoizedFunction,
    InoutParamsInCoroutine,
    InoutParamsSpecial,
    InoutParamsMixByrefDEPRECATED,
    InoutParamsMemoize,
    InoutParamsRetByRefDEPRECATED,
    ReadingFromAppend,
    ConstAttributeProhibitedDEPRECATED,
    RetiredError3049DEPRECATED,
    InoutArgumentBadExpr,
    MutableParamsOutsideOfSyncDEPRECATED,
    MutableAsyncMethodDEPRECATED,
    MutableMethodsMustBeReactive,
    MutableAttributeOnFunction,
    MutableReturnAnnotatedDeclsMustBeReactive,
    IllegalDestructor,
    ConditionallyReactiveFunctionDEPRECATED,
    MultipleConditionallyReactiveAnnotations,
    ConditionallyReactiveAnnotationInvalidArguments,
    MissingReactivityForConditionDEPRECATED,
    MultipleReactivityAnnotationsDEPRECATED,
    RxIsEnabledInvalidLocation,
    MaybeRxInvalidLocation,
    NoOnlyrxIfRxfuncForRxIfArgs,
    CoroutineInConstructor,
    IllegalReturnByRefDEPRECATED,
    IllegalByRefExprDEPRECATED,
    VariadicByRefParamDEPRECATED,
    MaybeMutableAttributeOnFunction,
    ConflictingMutableAndMaybeMutableAttributes,
    MaybeMutableMethodsMustBeReactive,
    RequiresFinalClass,
    InterfaceUsesTrait,
    NonstaticMethodInAbstractFinalClass,
    MutableOnStaticDEPRECATED,
    ClassnameConstInstanceOfDEPRECATED,
    ByRefParamOnConstructDEPRECATED,
    ByRefDynamicCallDEPRECATED,
    ByRefPropertyDEPRECATED,
    ByRefCallDEPRECATED,
    SwitchNonTerminalDefault,
    SwitchMultipleDefault,
    RepeatedRecordFieldName,
    PhpLambdaDisallowed,
    EntryPointArguments,
    VariadicMemoize,
    AbstractMethodMemoize,
}
impl TrivialDrop for NastCheck {}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    FromOcamlRep,
    FromOcamlRepIn,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub enum Typing {
    AbstractClassFinalDEPRECATED = 4001,
    UninstantiableClass,
    AnonymousRecursiveDEPRECATED,
    AnonymousRecursiveCallDEPRECATED,
    ArrayAccessRead,
    ArrayAppend,
    ArrayCast,
    ArrayGetArity,
    BadCall,
    ClassArityDEPRECATED,
    ConstMutation,
    ConstructorNoArgs,
    CyclicClassDef,
    CyclicTypedef,
    DiscardedAwaitable,
    IssetEmptyInStrict,
    DynamicYieldPrivateDEPRECATED,
    EnumConstantTypeBad,
    EnumSwitchNonexhaustive,
    EnumSwitchNotConst,
    EnumSwitchRedundant,
    EnumSwitchRedundantDefault,
    EnumSwitchWrongClass,
    EnumTypeBad,
    EnumTypeTypedefMixedDEPRECATED,
    ExpectedClass,
    ExpectedLiteralFormatString,
    ExpectedStaticIntDEPRECATED,
    ExpectedTparam,
    ExpectingReturnTypeHint,
    ExpectingReturnTypeHintSuggestDEPRECATED,
    ExpectingTypeHint,
    ExpectingTypeHintVariadic,
    RetiredError4034DEPRECATED,
    ExtendFinal,
    FieldKinds,
    FieldMissingDEPRECATED,
    FormatString,
    FunArityMismatch,
    FunTooFewArgs,
    FunTooManyArgs,
    FunUnexpectedNonvariadic,
    FunVariadicityHhVsPhp56,
    GenaExpectsArrayDEPRECATED,
    GenericArrayStrict,
    GenericStatic,
    ImplementAbstract,
    InterfaceFinalDEPRECATED,
    InvalidShapeFieldConst,
    InvalidShapeFieldLiteral,
    InvalidShapeFieldName,
    InvalidShapeFieldType,
    MemberNotFound,
    MemberNotImplemented,
    MissingAssign,
    MissingConstructor,
    MissingField,
    NegativeTupleIndexDEPRECATED,
    SelfOutsideClass,
    NewStaticInconsistent,
    StaticOutsideClass,
    NonObjectMemberRead,
    NullContainer,
    NullMemberRead,
    NullableParameterDEPRECATED,
    OptionReturnOnlyTypehint,
    ObjectString,
    OptionMixed,
    OverflowDEPRECATED,
    OverrideFinal,
    OverridePerTrait,
    PairArityDEPRECATED,
    AbstractCall,
    ParentInTrait,
    ParentOutsideClass,
    ParentUndefined,
    PreviousDefault,
    PrivateClassMeth,
    PrivateInstMeth,
    PrivateOverride,
    ProtectedClassMeth,
    ProtectedInstMeth,
    ReadBeforeWrite,
    ReturnInVoid,
    ShapeFieldClassMismatch,
    ShapeFieldTypeMismatch,
    ShouldBeOverride,
    SketchyNullCheckDEPRECATED,
    SketchyNullCheckPrimitiveDEPRECATED,
    SmemberNotFound,
    StaticDynamic,
    StaticOverflowDEPRECATED,
    RetiredError4093DEPRECATED,
    ThisInStaticDEPRECATED,
    ThisVarOutsideClass,
    TraitFinalDEPRECATED,
    TupleArityDEPRECATED,
    TupleArityMismatchDEPRECATED,
    TupleIndexTooLargeDEPRECATED,
    TupleSyntax,
    TypeArityMismatch,
    TypeParamArityDEPRECATED,
    RetiredError4103DEPRECATED,
    TypingTooFewArgs,
    TypingTooManyArgs,
    UnboundGlobal,
    UnboundNameTyping,
    UndefinedField,
    UndefinedParent,
    UnifyError,
    UnsatisfiedReq,
    Visibility,
    VisibilityExtends,
    VoidParameterDEPRECATED,
    WrongExtendKind,
    GenericUnify,
    NullsafeNotNeeded,
    TrivialStrictEq,
    VoidUsage,
    DeclaredCovariant,
    DeclaredContravariant,
    UnsetInStrictDEPRECATED,
    StrictMembersNotKnown,
    ErasedGenericAtRuntime,
    DynamicClassDEPRECATED,
    AttributeTooManyArguments,
    AttributeParamType,
    DeprecatedUse,
    AbstractConstUsage,
    CannotDeclareConstant,
    CyclicTypeconst,
    NullsafePropertyWriteContext,
    NoreturnUsage,
    ThisLvalueDEPRECATED,
    UnsetNonidxInStrict,
    InvalidShapeFieldNameEmpty,
    InvalidShapeFieldNameNumberDEPRECATED,
    ShapeFieldsUnknown,
    InvalidShapeRemoveKey,
    MissingOptionalFieldDEPRECATED,
    ShapeFieldUnset,
    AbstractConcreteOverride,
    LocalVariableModifedAndUsed,
    LocalVariableModifedTwice,
    AssignDuringCase,
    CyclicEnumConstraint,
    UnpackingDisallowed,
    InvalidClassname,
    InvalidMemoizedParam,
    IllegalTypeStructure,
    NotNullableCompareNullTrivial,
    ClassPropertyOnlyStaticLiteralDEPRECATED,
    AttributeTooFewArguments,
    ReferenceExprDEPRECATED,
    UnificationCycle,
    KeysetSet,
    EqIncompatibleTypes,
    ContravariantThis,
    InstanceofAlwaysFalseDEPRECATED,
    InstanceofAlwaysTrueDEPRECATED,
    AmbiguousMember,
    InstanceofGenericClassnameDEPRECATED,
    RequiredFieldIsOptional,
    FinalProperty,
    ArrayGetWithOptionalField,
    UnknownFieldDisallowedInShape,
    NullableCast,
    PassByRefAnnotationMissingDEPRECATED,
    NonCallArgumentInSuspend,
    NonCoroutineCallInSuspend,
    CoroutineCallOutsideOfSuspend,
    FunctionIsNotCoroutine,
    CoroutinnessMismatch,
    ExpectingAwaitableReturnTypeHint,
    ReffinessInvariantDEPRECATED,
    DollardollarLvalue,
    StaticMethodOnInterfaceDEPRECATED,
    DuplicateUsingVar,
    IllegalDisposable,
    EscapingDisposable,
    PassByRefAnnotationUnexpectedDEPRECATED,
    InoutAnnotationMissing,
    InoutAnnotationUnexpected,
    InoutnessMismatch,
    StaticSyntheticMethod,
    TraitReuse,
    InvalidNewDisposable,
    EscapingDisposableParameter,
    AcceptDisposableInvariant,
    InvalidDisposableHint,
    XhpRequired,
    EscapingThis,
    IllegalXhpChild,
    MustExtendDisposable,
    InvalidIsAsExpressionHint,
    AssigningToConst,
    SelfConstParentNot,
    ParentConstSelfNotDEPRECATED,
    PartiallyValidIsAsExpressionHintDEPRECATED,
    NonreactiveFunctionCall,
    NonreactiveIndexing,
    ObjSetReactive,
    FunReactivityMismatch,
    OverridingPropConstMismatch,
    InvalidReturnDisposable,
    InvalidDisposableReturnHint,
    ReturnDisposableMismatch,
    InoutArgumentBadType,
    InconsistentUnsetDEPRECATED,
    ReassignMutableVar,
    InvalidFreezeTarget,
    InvalidFreezeUse,
    FreezeInNonreactiveContext,
    MutableCallOnImmutable,
    MutableArgumentMismatch,
    InvalidMutableReturnResult,
    MutableReturnResultMismatch,
    NonreactiveCallFromShallow,
    EnumTypeTypedefNonnull,
    RxEnabledInNonRxContext,
    RxEnabledInLambdasDEPRECATED,
    AmbiguousLambda,
    EllipsisStrictMode,
    UntypedLambdaStrictMode,
    BindingRefInArrayDEPRECATED,
    EchoInReactiveContext,
    SuperglobalInReactiveContext,
    StaticPropertyInReactiveContext,
    StaticInReactiveContextDEPRECATED,
    GlobalInReactiveContextDEPRECATED,
    WrongExpressionKindAttribute,
    AttributeClassNoConstructorArgsDEPRECATED,
    InvalidTypeForOnlyrxIfRxfuncParameter,
    MissingAnnotationForOnlyrxIfRxfuncParameter,
    CannotReturnBorrowedValueAsImmutable,
    DeclOverrideMissingHint,
    InvalidConditionallyReactiveCall,
    ExtendSealed,
    SealedFinalDEPRECATED,
    ComparisonInvalidTypes,
    OptionVoidDEPRECATED,
    MutableInNonreactiveContext,
    InvalidArgumentOfRxMutableFunction,
    LetVarImmutabilityViolation,
    UnsealableDEPRECATED,
    ReturnVoidToRxMismatch,
    ReturnsVoidToRxAsNonExpressionStatement,
    NonawaitedAwaitableInReactiveContext,
    ShapesKeyExistsAlwaysTrue,
    ShapesKeyExistsAlwaysFalse,
    ShapesMethodAccessWithNonExistentField,
    NonClassMember,
    PassingArrayCellByRefDEPRECATED,
    CallSiteReactivityMismatch,
    RxParameterConditionMismatch,
    AmbiguousObjectAccess,
    ExtendPPL,
    ReassignMaybeMutableVar,
    MaybeMutableArgumentMismatch,
    ImmutableArgumentMismatch,
    ImmutableCallOnMutable,
    InvalidCallMaybeMutable,
    MutabilityMismatch,
    InvalidPPLCall,
    InvalidPPLStaticCall,
    TypeTestInLambdaDEPRECATED,
    InvalidTraversableInRx,
    ReassignMutableThis,
    MutableExpressionAsMultipleMutableArguments,
    InvalidUnsetTargetInRx,
    CoroutineOutsideExperimental,
    PPLMethPointer,
    InvalidTruthinessTestDEPRECATED,
    RePrefixedNonString,
    BadRegexPattern,
    SketchyTruthinessTestDEPRECATED,
    LateInitWithDefault,
    OverrideMemoizeLSB,
    ClassVarTypeGenericParam,
    InvalidSwitchCaseValueType,
    StringCast,
    BadLateInitOverride,
    EscapingMutableObject,
    OverrideLSB,
    MultipleConcreteDefs,
    MoveInNonreactiveContext,
    InvalidMoveUse,
    InvalidMoveTarget,
    IgnoredResultOfFreezeDEPRECATED,
    IgnoredResultOfMoveDEPRECATED,
    UnexpectedTy,
    UnserializableType,
    InconsistentMutability,
    InvalidMutabilityFlavorInAssignment,
    OptionNull,
    UnknownObjectMember,
    UnknownType,
    InvalidArrayKeyRead,
    ReferenceExprNotFunctionArgDEPRECATED,
    RedundantRxCondition,
    RedeclaringMissingMethod,
    InvalidEnforceableTypeArgument,
    RequireArgsReify,
    TypecheckerTimeout,
    InvalidReifiedArgument,
    GenericsNotAllowed,
    InvalidNewableTypeArgument,
    InvalidNewableTypeParamConstraints,
    NewWithoutNewable,
    NewStaticClassReified,
    MemoizeReified,
    ConsistentConstructReified,
    MethodVariance,
    MissingXhpRequiredAttr,
    BadXhpAttrRequiredOverride,
    ReifiedTparamVariadicDEPRECATED,
    UnresolvedTypeVariable,
    InvalidSubString,
    InvalidArrayKeyConstraint,
    OverrideNoDefaultTypeconst,
    ShapeAccessWithNonExistentField,
    DisallowPHPArraysAttr,
    TypeConstraintViolation,
    IndexTypeMismatch,
    ExpectedStringlike,
    TypeConstantMismatch,
    TypeConstantRedeclarationDEPRECATED,
    ConstantDoesNotMatchEnumType,
    EnumConstraintMustBeArraykey,
    EnumSubtypeMustHaveCompatibleConstraint,
    ParameterDefaultValueWrongType,
    NewtypeAliasMustSatisfyConstraint,
    BadFunctionTypevar,
    BadClassTypevar,
    BadMethodTypevar,
    MissingReturnInNonVoidFunction,
    InoutReturnTypeMismatch,
    ClassConstantValueDoesNotMatchHint,
    ClassPropertyInitializerTypeDoesNotMatchHint,
    BadDeclOverride,
    BadMethodOverride,
    BadEnumExtends,
    XhpAttributeValueDoesNotMatchHint,
    TraitPropConstClass,
    EnumUnderlyingTypeMustBeArraykey,
    ClassGetReified,
    RequireGenericExplicit,
    ClassConstantTypeMismatch,
    PocketUniversesExpansion,
    PocketUniversesTyping,
    RecordInitValueDoesNotMatchHint,
    AbstractTconstNotAllowed,
    NewAbstractRecord,
    RecordMissingRequiredField,
    RecordUnknownField,
    CyclicRecordDef,
    InvalidDestructure,
    StaticMethWithClassReifiedGeneric,
    SplatArrayRequired,
    SplatArrayVariadic,
    ExceptionOccurred,
    InvalidReifiedFunctionPointer,
    BadFunctionPointerConstruction,
    NotARecord,
    TraitReuseInsideClass,
    RedundantGeneric,
    PocketUniversesInvalidUpperBounds,
    PocketUniversesRefinement,
    PocketUniversesReservedSyntax,
    ArrayAccessWrite,
    InvalidArrayKeyWrite,
    NullMemberWrite,
    NonObjectMemberWrite,
    ConcreteConstInterfaceOverride,
    MethCallerTrait,
    PocketUniversesAttributes,
    DuplicateInterface,
    TypeParameterNameAlreadyUsedNonShadow,
    IllegalInformationFlow,
    ContextImplicitPolicyLeakage,
}
impl TrivialDrop for Typing {}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    FromOcamlRep,
    FromOcamlRepIn,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub enum Init {
    ForwardCompatibilityNotCurrent = 8001,
    ForwardCompatibilityBelowMinimum,
}
impl TrivialDrop for Init {}
