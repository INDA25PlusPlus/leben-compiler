use leben_parsable::*;
use serde::{Deserialize, Serialize};

literals! {
    pub(crate) struct CommentToken = b"#";
    pub(crate) struct CommaToken = b",";
    pub(crate) struct SemicolonToken = b";";
    pub(crate) struct PeriodToken = b".";
    pub(crate) struct ColonToken = b":";
    pub(crate) struct OpeningParenToken = b"(";
    pub(crate) struct ClosingParenToken = b")";
    pub(crate) struct OpeningBracketToken = b"[";
    pub(crate) struct ClosingBracketToken = b"]";
    pub(crate) struct OpeningBraceToken = b"{";
    pub(crate) struct ClosingBraceToken = b"}";
    pub(crate) struct RefToken = b"&";
    pub(crate) struct DerefToken = b"*";
    pub(crate) struct SingleQuoteToken = b"'";
    pub(crate) struct DoubleQuoteToken = b"\"";
    pub(crate) struct PublicToken = b"public";
    pub(crate) struct RootToken = b"root";
    pub(crate) struct ModuleToken = b"module";
    pub(crate) struct StaticToken = b"static";
    pub(crate) struct StructToken = b"struct";
    pub(crate) struct MethodToken = b"method";
    pub(crate) struct ThisToken = b"this";
    pub(crate) struct FunctionToken = b"function";
    pub(crate) struct BuiltinFunctionCallPrefixToken = b"@";
    pub(crate) struct IfToken = b"if";
    pub(crate) struct ElseIfToken = b"else if";
    pub(crate) struct ElseToken = b"else";
    pub(crate) struct LoopToken = b"loop";
    pub(crate) struct ReturnToken = b"return";
    pub(crate) struct BreakToken = b"break";
    pub(crate) struct LetToken = b"let";
    pub(crate) struct MutableToken = b"mut";
    pub(crate) struct UndefinedToken = b"undefined";
    pub(crate) struct OrToken = b"or";
    pub(crate) struct XorToken = b"xor";
    pub(crate) struct AndToken = b"and";
    pub(crate) struct EqCompToken = b"==";
    pub(crate) struct NeqCompToken = b"!=";
    pub(crate) struct LtToken = b"<";
    pub(crate) struct GtToken = b">";
    pub(crate) struct LteqToken = b"<=";
    pub(crate) struct GteqToken = b">=";
    pub(crate) struct AddToken = b"+";
    pub(crate) struct SubToken = b"-";
    pub(crate) struct MultToken = b"*";
    pub(crate) struct DivToken = b"/";
    pub(crate) struct MinusToken = b"-";
    pub(crate) struct InvertToken = b"~";
    pub(crate) struct NotToken = b"!";
    pub(crate) struct EqToken = b"=";
    pub(crate) struct OrEqToken = b"or=";
    pub(crate) struct XorEqToken = b"xor=";
    pub(crate) struct AndEqToken = b"and=";
    pub(crate) struct AddEqToken = b"+=";
    pub(crate) struct SubEqToken = b"-=";
    pub(crate) struct MultEqToken = b"*=";
    pub(crate) struct DivEqToken = b"/=";
    pub(crate) struct BinaryPrefixToken = b"0b";
    pub(crate) struct HexadecimalPrefixToken = b"0x";
    pub(crate) struct FloatScientificPrefixToken = b"0.1";
    pub(crate) struct ScientificExponentToken = b"E";
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct SourceFile {
    pub module: Module,
    _0: EndOfStream,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Module {
    _0: Ows,
    pub declarations: ZeroPlus<ModulePart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ModulePart {
    pub declaration: GlobalDeclaration,
    _0: Ows,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum GlobalDeclaration {
    Comment(Ignore<Comment>),
    ModuleDeclaration(ModuleDeclaration),
    StaticDeclaration(StaticDeclaration),
    MethodDeclaration(MethodDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    StructDeclaration(StructDeclaration),
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Comment {
    _0: CommentToken,
    pub comment: Span<ZeroPlus<NonNewlineChar>>,
    _2: NewlineChar,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ModuleDeclaration {
    _0: ModuleToken,
    _1: Ws,
    pub module_name: ModuleName,
    _2: Ows,
    _3: OpeningBraceToken,
    pub module: Module,
    _4: ClosingBraceToken,
}

type ModuleName = IdentifierPath;



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StaticDeclaration {
    pub visibility: VisibilitySpecifier,
    _0: StaticToken,
    _1: Ws,
    pub declaration: VariableDeclaration,
    _2: Ows,
    _3: SemicolonToken,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodDeclaration {
    pub visibility: VisibilitySpecifier,
    _0: MethodToken,
    _1: Ws,
    pub function_name: MethodName,
    _2: Ws,
    _3: OpeningParenToken,
    _4: Ows,
    pub members: Option<MethodParameterList>,
    _5: ClosingParenToken,
    _6: Ows,
    pub return_type_and_body: FunctionReturnTypeAndBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodName {
    pub type_name: TypeName,
    _0: Ows,
    _1: PeriodToken,
    _2: Ows,
    pub function_name: FunctionName,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodParameterList {
    _0: ThisToken,
    _1: Ows,
    pub parameters: ZeroPlus<MethodParameterListPart>,
    _2: Option<CommaWs>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodParameterListPart {
    _0: CommaToken,
    _1: Ows,
    pub parameter: FunctionParameter,
    _2: Ows,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct CommaWs {
    _0: CommaToken,
    _1: Ows,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionDeclaration {
    pub visibility: VisibilitySpecifier,
    _0: FunctionToken,
    _1: Ws,
    pub function_name: FunctionName,
    _2: Ows,
    _3: OpeningParenToken,
    _4: Ows,
    pub members: Option<FunctionParameterList>,
    _5: ClosingParenToken,
    _6: Ows,
    pub return_type_and_body: FunctionReturnTypeAndBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionParameterList {
    pub members: ZeroPlus<FunctionParameterListPart>,
    pub last_member: FunctionParameter,
    _0: Option<WsComma>,
    _1: Ows,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionParameterListPart {
    pub member: FunctionParameter,
    _0: WsComma,
    _1: Ows,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionParameter {
    pub name: FunctionParameterName,
    _0: Ows,
    _1: ColonToken,
    _2: Ows,
    pub type_ref: TypeReference,
}

type FunctionName = Identifier;

type FunctionParameterName = Identifier;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionReturnTypeAndBody {
    pub return_type: Option<FunctionReturnType>,
    pub body: CompoundStatement,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionReturnType {
    _0: ColonToken,
    _1: Ows,
    pub type_ref: TypeReference,
    _2: Ws,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructDeclaration {
    pub visibility: VisibilitySpecifier,
    _0: StructToken,
    _1: Ws,
    pub type_name: TypeName,
    _2: Ws,
    _3: OpeningBraceToken,
    _4: Ows,
    pub members: Option<StructMemberList>,
    _5: ClosingBraceToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructMemberList {
    pub members: ZeroPlus<StructMemberListPart>,
    pub last_member: StructMember,
    _0: Option<WsComma>,
    _1: Ows,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructMemberListPart {
    pub member: StructMember,
    _0: WsComma,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructMember {
    pub name: StructMemberName,
    _0: Ows,
    _1: ColonToken,
    _2: Ows,
    pub type_ref: TypeReference,
}

type StructMemberName = Identifier;



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum Statement {
    Compound(CompoundStatement),
    If(IfStatement),
    Loop(LoopStatement),
    Return(ReturnStatement),
    Break(BreakStatement),
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
    Expr(ExprStatement),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct CompoundStatement {
    _0: OpeningBraceToken,
    _1: Ows,
    pub statements: ZeroPlus<CompoundStatementPart>,
    _2: ClosingBraceToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct CompoundStatementPart {
    pub statement: Statement,
    _0: Ows,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IfStatement {
    _0: IfToken,
    pub if_clause: IfClause,
    pub else_if_clauses: ZeroPlus<ElseIfClause>,
    pub else_clause: Option<ElseClause>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IfClause {
    _0: Ws,
    pub expr: Expr,
    _1: Ws,
    pub block: CompoundStatement,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ElseIfClause {
    _0: Ws,
    _1: ElseIfToken,
    pub if_clause: IfClause,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ElseClause {
    _0: Ws,
    _1: ElseToken,
    _2: Ws,
    pub block: CompoundStatement,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LoopStatement {
    _0: LoopToken,
    _1: Ws,
    pub block: CompoundStatement,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ReturnStatement {
    _0: ReturnToken,
    pub expr: Option<ReturnStatementExpr>,
    _1: Ows,
    _2: SemicolonToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ReturnStatementExpr {
    _0: Ws,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct BreakStatement {
    _0: BreakToken,
    _1: Ows,
    _2: SemicolonToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct VariableDeclarationStatement {
    _0: LetToken,
    _1: Ws,
    pub declaration: VariableDeclaration,
    _2: Ows,
    _3: SemicolonToken,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AssignmentStatement {
    pub lhs: PrefixAssigneeExpr,
    _0: Ows,
    pub op: AssignmentOperator,
    _1: Ows,
    pub rhs: Expr,
    _2: Ows,
    _3: SemicolonToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PrefixAssigneeExpr {
    Prefix(PrefixAssigneeExprInner),
    Postfix(PostfixAssigneeExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PrefixAssigneeExprInner {
    _0: DerefToken,
    _1: Ows,
    pub expr: Box<PrefixAssigneeExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PostfixAssigneeExpr {
    pub variable: VariableReference,
    pub tail: Option<PostfixAssigneeExprTail>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PostfixAssigneeExprTail {
    Index(AssigneeIndexExpr),
    Member(AssigneeMemberExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AssigneeIndexExpr {
    _0: Ows,
    _1: OpeningBracketToken,
    _2: Ows,
    pub index_expr: Expr,
    _3: ClosingBracketToken,
    pub tail: Option<Box<PostfixAssigneeExprTail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AssigneeMemberExpr {
    _0: Ows,
    _1: PeriodToken,
    _2: Ows,
    pub path: IdentifierPath,
    pub tail: Option<Box<PostfixAssigneeExprTail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum AssignmentOperator {
    Eq(EqToken),
    OrEq(OrEqToken),
    XorEq(XorEqToken),
    AndEq(AndEqToken),
    AddEq(AddEqToken),
    SubEq(SubEqToken),
    MultEq(MultEqToken),
    DivEq(DivEqToken),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ExprStatement {
    pub expr: Expr,
    _0: Ows,
    _1: SemicolonToken,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Expr {
    pub or_expr: OrExpr,
    _0: Ows,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum OrExpr {
    Or(OrExprInner),
    Xor(XorExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct OrExprInner {
    pub lhs: XorExpr,
    _0: Ws,
    _1: OrToken,
    _2: Ws,
    pub rhs: Box<OrExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum XorExpr {
    Xor(XorExprInner),
    And(AndExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct XorExprInner {
    pub lhs: AndExpr,
    _0: Ws,
    _1: XorToken,
    _2: Ws,
    pub rhs: Box<XorExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum AndExpr {
    And(AndExprInner),
    Equality(EqualityExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AndExprInner {
    pub lhs: EqualityExpr,
    _0: Ws,
    _1: AndToken,
    _2: Ws,
    pub rhs: Box<AndExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum EqualityExpr {
    Eq(EqExpr),
    Neq(NeqExpr),
    Comp(CompExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EqExpr {
    pub lhs: CompExpr,
    _0: Ows,
    _1: EqCompToken,
    _2: Ows,
    pub rhs: Box<EqualityExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct NeqExpr {
    pub lhs: CompExpr,
    _0: Ows,
    _1: NeqCompToken,
    _2: Ows,
    pub rhs: Box<EqualityExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum CompExpr {
    Lt(LtExpr),
    Gt(GtExpr),
    Lte(LteExpr),
    Gte(GteExpr),
    Additive(AdditiveExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LtExpr {
    pub lhs: AdditiveExpr,
    _0: Ows,
    _1: LtToken,
    _2: Ows,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct GtExpr {
    pub lhs: AdditiveExpr,
    _0: Ows,
    _1: GtToken,
    _2: Ows,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LteExpr {
    pub lhs: AdditiveExpr,
    _0: Ows,
    _1: LteqToken,
    _2: Ows,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct GteExpr {
    pub lhs: AdditiveExpr,
    _0: Ows,
    _1: GteqToken,
    _2: Ows,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum AdditiveExpr {
    Add(AddExpr),
    Sub(SubExpr),
    Multiplicative(MultiplicativeExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AddExpr {
    pub lhs: MultiplicativeExpr,
    _0: Ows,
    _1: AddToken,
    _2: Ows,
    pub rhs: Box<AdditiveExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct SubExpr {
    pub lhs: MultiplicativeExpr,
    _0: Ows,
    _1: SubToken,
    _2: Ows,
    pub rhs: Box<AdditiveExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum MultiplicativeExpr {
    Mult(MultExpr),
    Div(DivExpr),
    Prefix(PrefixExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MultExpr {
    pub lhs: PrefixExpr,
    _0: Ows,
    _1: MultToken,
    _2: Ows,
    pub rhs: Box<MultiplicativeExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct DivExpr {
    pub lhs: PrefixExpr,
    _0: Ows,
    _1: DivToken,
    _2: Ows,
    pub rhs: Box<MultiplicativeExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PrefixExpr {
    Ref(RefExpr),
    Deref(DerefExpr),
    Minus(MinusExpr),
    Invert(InvertExpr),
    Not(NotExpr),
    Postfix(PostfixExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct RefExpr {
    _0: RefToken,
    _1: Ows,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct DerefExpr {
    _0: DerefToken,
    _1: Ows,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MinusExpr {
    _0: MinusToken,
    _1: Ows,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct InvertExpr {
    _0: InvertToken,
    _1: Ows,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct NotExpr {
    _0: NotToken,
    _1: Ows,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PostfixExpr {
    pub inner: InnerExpr,
    pub tail: Option<PostfixExprTail>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PostfixExprTail {
    IndexExpr(IndexExpr),
    CallExpr(CallExpr),
    MemberExpr(MemberExpr),
    Inner(InnerExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IndexExpr {
    _0: Ows,
    _1: OpeningBracketToken,
    _2: Ows,
    pub index_expr: Box<Expr>,
    _3: ClosingBracketToken,
    pub tail: Option<Box<PostfixExprTail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct CallExpr {
    _0: Ows,
    _1: OpeningParenToken,
    _2: Ows,
    pub args: Option<FunctionArgumentList>,
    _3: ClosingParenToken,
    pub tail: Option<Box<PostfixExprTail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MemberExpr {
    _0: Ows,
    _1: PeriodToken,
    _2: Ows,
    pub path: IdentifierPath,
    pub tail: Option<Box<PostfixExprTail>>,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum InnerExpr {
    Enclosed(EnclosedExpr),
    StructInit(StructInitExpr),
    BuiltinFunctionCall(BuiltinFunctionCallExpr),
    LiteralInt(LiteralIntExpr),
    LiteralFloat(LiteralFloatExpr),
    LiteralString(LiteralStringExpr),
    VariableReference(VariableReference),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EnclosedExpr {
    _0: OpeningParenToken,
    pub expr: Box<Expr>,
    _1: ClosingParenToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitExpr {
    _0: OpeningBracketToken,
    _1: Ows,
    pub list: Option<StructInitList>,
    _2: ClosingBracketToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitList {
    pub entries: ZeroPlus<StructInitListPart>,
    pub last_entry: StructInitEntry,
    _0: Option<WsComma>,
    _1: Ows,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitListPart {
    pub entry: StructInitEntry,
    _0: WsComma,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitEntry {
    pub name: VariableName,
    _0: Ows,
    _1: EqToken,
    _2: Ows,
    pub expr: Box<Expr>,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct BuiltinFunctionCallExpr {
    _0: BuiltinFunctionCallPrefixToken,
    pub function: FunctionReference,
    _1: Ows,
    _2: OpeningParenToken,
    _3: Ows,
    pub args: Option<FunctionArgumentList>,
    _4: ClosingParenToken,
}

type FunctionReference = VariableReference;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionArgumentList {
    pub args: ZeroPlus<FunctionArgumentListPart>,
    pub last_arg: FunctionArgument,
    _0: Option<WsComma>,
    _1: Ows,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionArgumentListPart {
    pub arg: FunctionArgument,
    _0: WsComma,
}

type FunctionArgument = Box<Expr>;



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum LiteralIntExpr {
    Int(LiteralInt),
    IntBinary(LiteralIntBinary),
    IntHexadecimal(LiteralIntHexadecimal),
    IntChar(LiteralIntChar),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralInt {
    pub minus: Option<MinusToken>,
    pub digits: Span<OnePlus<Numerical>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralIntBinary {
    _0: BinaryPrefixToken,
    pub digits: Span<OnePlus<BinaryOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralIntHexadecimal {
    _0: HexadecimalPrefixToken,
    pub digits: Span<OnePlus<HexadecimalOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralIntChar {
    _0: SingleQuoteToken,
    pub char: CharChar,
    _1: SingleQuoteToken,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum LiteralFloatExpr {
    Float(LiteralFloat),
    FloatScientific(LiteralFloatScientific),
    FloatBinary(LiteralFloatBinary),
    FloatHexadecimal(LiteralFloatHexadecimal),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralFloat {
    pub minus: Option<MinusToken>,
    pub before_decimal: Span<OnePlus<Numerical>>,
    _0: PeriodToken,
    pub after_decimal: Span<OnePlus<Numerical>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralFloatScientific {
    pub mantissa_minus: Option<MinusToken>,
    _0: FloatScientificPrefixToken,
    pub mantissa: Span<OnePlus<Numerical>>,
    _1: ScientificExponentToken,
    pub exponent_minus: Option<MinusToken>,
    pub exponent: Span<OnePlus<Numerical>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralFloatBinary {
    pub mantissa_minus: Option<MinusToken>,
    _0: BinaryPrefixToken,
    pub mantissa: Span<OnePlus<BinaryOrUnderscore>>,
    _1: ScientificExponentToken,
    pub exponent_minus: Option<MinusToken>,
    pub exponent: Span<OnePlus<BinaryOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralFloatHexadecimal {
    pub mantissa_minus: Option<MinusToken>,
    _0: HexadecimalPrefixToken,
    pub mantissa: Span<OnePlus<HexadecimalOrUnderscore>>,
    _1: ScientificExponentToken,
    pub exponent_minus: Option<MinusToken>,
    pub exponent: Span<OnePlus<HexadecimalOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralStringExpr {
    _0: DoubleQuoteToken,
    pub string: LiteralString,
    _1: DoubleQuoteToken,
}

type LiteralString = Span<ZeroPlus<StringChar>>;



type VisibilitySpecifier = Option<Public>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Public {
    _0: PublicToken,
    _1: Ws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct VariableDeclaration {
    pub mutability_specifier: MutabilitySpecifier,
    pub variable_name: VariableName,
    _0: Ws,
    _1: ColonToken,
    _2: Ows,
    pub type_reference: TypeReference,
    _3: Ws,
    _4: EqToken,
    _5: Ows,
    pub rhs: VariableDeclarationRhs,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum VariableDeclarationRhs {
    Undefined(UndefinedToken),
    Expr(Expr),
}

type MutabilitySpecifier = Option<Mutable>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Mutable {
    _0: MutableToken,
    _1: Ws,
}

type VariableName = Identifier;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum VariableReference {
    GlobalIdentifierPath(GlobalIdentifierPath),
    IdentifierPath(IdentifierPath),
}



type TypeName = Identifier;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum TypeReference {
    GlobalIdentifierPath(GlobalIdentifierPath),
    IdentifierPath(IdentifierPath),
    PrefixedTypeReference(PrefixedTypeReference),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PrefixedTypeReference {
    _0: RefToken,
    _1: Ows,
    type_reference: Box<TypeReference>,
}



type Identifier = Span<IdentifierInner>;

type Ows = Ignore<ZeroPlus<WhitespaceChar>>;

type Ws = Ignore<OnePlus<WhitespaceChar>>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct WsComma {
    _0: Ows,
    _1: CommaToken,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct GlobalIdentifierPath {
    _0: RootToken,
    pub path_parts: OnePlus<PathPart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IdentifierPath {
    pub identifier: Identifier,
    pub path_parts: ZeroPlus<PathPart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PathPart {
    _0: Ows,
    _1: PeriodToken,
    _2: Ows,
    pub identifier: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IdentifierInner {
    initial: IdentifierInitialChar,
    others: ZeroPlus<IdentifierChar>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum IdentifierInitialChar {
    Alphabetical(Alphabetical),
    Underscore(CharLiteral<b'_'>)
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum IdentifierChar {
    Alphabetical(Alphabetical),
    Numerical(Numerical),
    Underscore(CharLiteral<b'_'>)
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum CharChar {
    DoubleQuote(CharLiteral<b'"'>),
    EscapedSingleQuote(EscapedSingleQuote),
    EscapedChar(EscapedChar),
    NonEscapedChar(NonEscapedChar),
    NonAsciiByte(NonAsciiByte),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum StringChar {
    EscapedDoubleQuote(EscapedDoubleQuote),
    SingleQuote(CharLiteral<b'\''>),
    EscapedChar(EscapedChar),
    NonEscapedChar(NonEscapedChar),
    NonAsciiByte(NonAsciiByte),
}

type NonAsciiByte = CharRange<b'\x80', b'\xff'>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedSingleQuote {
    #[literal = b"\\'"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedDoubleQuote {
    #[literal = b"\\\""] _0: (),
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum WhitespaceChar { Space(CharLiteral<b' '>), Tab(CharLiteral<b'\t'>), NewlineChar(NewlineChar) }

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum NewlineChar { Crlf(Crlf), Lf(CharLiteral<b'\n'>), Cr(CharLiteral<b'\r'>) }

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Crlf {
    #[literal = b"\r\n"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum NonNewlineChar { NonEscapedChar(NonEscapedChar), DoubleQuote(CharLiteral<b'"'>), SingleQuote(CharLiteral<b'\''>) }

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum NonEscapedChar { 
    Tab(CharLiteral<b'\t'>), 
    Range1(CharRange<b' ', b'!'>), 
    Range2(CharRange<b'#', b'&'>), 
    Range3(CharRange<b'(', b'~'>),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum EscapedChar { 
    Backslash(EscapedBackslash), 
    Newline(EscapedNewline),
    CarriageReturn(EscapedCarriageReturn),
    Tab(EscapedTab),
    Backspace(EscapedBackspace),
    Formfeed(EscapedFormfeed),
    VerticalTab(EscapedVerticalTab),
    Null(EscapedNull),
    Byte(EscapedByte),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedBackslash { 
    #[literal = b"\\\\"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedNewline { 
    #[literal = b"\\n"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedCarriageReturn { 
    #[literal = b"\\r"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedTab { 
    #[literal = b"\\t"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedBackspace { 
    #[literal = b"\\b"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedFormfeed { 
    #[literal = b"\\f"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedVerticalTab { 
    #[literal = b"\\v"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedNull { 
    #[literal = b"\\0"] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EscapedByte { 
    #[literal = b"\\x"] _0: (),
    pub digit_0: Hexadecimal,
    pub digit_1: Hexadecimal,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum Alphabetical { LowerCase(LowerCaseAlphabetical), UpperCase(UpperCaseAlphabetical) }

type LowerCaseAlphabetical = CharRange<b'a', b'z'>;

type UpperCaseAlphabetical = CharRange<b'A', b'Z'>;

type Numerical = CharRange<b'0', b'9'>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum BinaryOrUnderscore { Binary(Binary), Underscore(CharLiteral<b'_'>) }

type Binary = CharRange<b'0', b'1'>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum HexadecimalOrUnderscore { Hexadecimal(Hexadecimal), Underscore(CharLiteral<b'_'>) }

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum Hexadecimal { ZeroToNine(Numerical), LowerCaseAToF(LowerCaseAToF), UpperCaseAToF(UpperCaseAToF) }

type LowerCaseAToF = CharRange<b'a', b'f'>;

type UpperCaseAToF = CharRange<b'A', b'F'>;
