use leben_parsable::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct SourceFile {
    pub module: Module,
    _0: EndOfStream,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Module {
    _0: Ws,
    pub declarations: ZeroPlus<ModulePart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ModulePart {
    pub declaration: GlobalDeclaration,
    _0: Ws,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum GlobalDeclaration {
    Comment(Comment),
    ModuleDeclaration(ModuleDeclaration),
    StaticDeclaration(StaticDeclaration),
    MethodDeclaration(MethodDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    StructDeclaration(StructDeclaration),
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Comment {
    _0: CharLiteral<b'#'>,
    pub comment: Span<ZeroPlus<NonNewlineChar>>,
    _2: NewlineChar,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ModuleDeclaration {
    #[literal = b"module"] _0: (),
    _1: Fws,
    pub module_name: ModuleName,
    _2: Ws,
    _3: CharLiteral<b'{'>,
    pub module: Module,
    _4: CharLiteral<b'}'>,
}

type ModuleName = IdentifierPath;



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StaticDeclaration {
    pub visibility: VisibilitySpecifier,
    #[literal = b"static"] _0: (),
    _1: Fws,
    pub declaration: VariableDeclaration,
    _2: Ws,
    _3: CharLiteral<b';'>,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodDeclaration {
    pub visibility: VisibilitySpecifier,
    #[literal = b"method"] _0: (),
    _1: Fws,
    pub function_name: MethodName,
    _2: Fws,
    _3: CharLiteral<b'('>,
    _4: Ws,
    pub members: Option<MethodParameterList>,
    _5: CharLiteral<b')'>,
    _6: Ws,
    pub return_type_and_body: FunctionReturnTypeAndBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodName {
    pub type_name: TypeName,
    _0: Ws,
    _1: CharLiteral<b'.'>,
    _2: Ws,
    pub function_name: FunctionName,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodParameterList {
    #[literal = b"this"] _0: (),
    _1: Ws,
    pub parameters: ZeroPlus<MethodParameterListPart>,
    _2: Option<CommaWs>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MethodParameterListPart {
    _0: CharLiteral<b','>,
    _1: Ws,
    pub parameter: FunctionParameter,
    _2: Ws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct CommaWs {
    _0: CharLiteral<b','>,
    _1: Ws,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionDeclaration {
    pub visibility: VisibilitySpecifier,
    #[literal = b"function"] _0: (),
    _1: Fws,
    pub function_name: FunctionName,
    _2: Ws,
    _3: CharLiteral<b'('>,
    _4: Ws,
    pub members: Option<FunctionParameterList>,
    _5: CharLiteral<b')'>,
    _6: Ws,
    pub return_type_and_body: FunctionReturnTypeAndBody,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionParameterList {
    pub members: ZeroPlus<FunctionParameterListPart>,
    pub last_member: FunctionParameter,
    _0: Option<WsComma>,
    _1: Ws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionParameterListPart {
    pub member: FunctionParameter,
    _0: WsComma,
    _1: Ws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionParameter {
    pub name: FunctionParameterName,
    _0: Ws,
    _1: CharLiteral<b':'>,
    _2: Ws,
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
    _0: CharLiteral<b':'>,
    _1: Ws,
    pub type_ref: TypeReference,
    _2: Fws,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructDeclaration {
    pub visibility: VisibilitySpecifier,
    #[literal = b"struct"] _0: (),
    _1: Fws,
    pub type_name: TypeName,
    _2: Fws,
    _3: CharLiteral<b'{'>,
    _4: Ws,
    pub members: Option<StructMemberList>,
    _5: CharLiteral<b'}'>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructMemberList {
    pub members: ZeroPlus<StructMemberListPart>,
    pub last_member: StructMember,
    _0: Option<WsComma>,
    _1: Ws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructMemberListPart {
    pub member: StructMember,
    _0: WsComma,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructMember {
    pub name: StructMemberName,
    _0: Ws,
    _1: CharLiteral<b':'>,
    _2: Ws,
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
    _0: CharLiteral<b'{'>,
    _1: Ws,
    pub statements: ZeroPlus<CompoundStatementPart>,
    _2: CharLiteral<b'}'>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct CompoundStatementPart {
    pub statement: Statement,
    _0: Ws,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IfStatement {
    #[literal = b"if"] _0: (),
    pub if_clause: IfClause,
    pub else_if_clauses: ZeroPlus<ElseIfClause>,
    pub else_clause: Option<ElseClause>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IfClause {
    _0: Fws,
    pub expr: Expr,
    _1: Fws,
    pub block: CompoundStatement,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ElseIfClause {
    _0: Fws,
    #[literal = b"else if"] _1: (),
    pub if_clause: IfClause,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ElseClause {
    _0: Fws,
    #[literal = b"else if"] _1: (),
    _2: Fws,
    pub block: CompoundStatement,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LoopStatement {
    #[literal = b"loop"] _0: (),
    _1: Fws,
    pub block: CompoundStatement,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ReturnStatement {
    #[literal = b"return"] _0: (),
    pub expr: Option<ReturnStatementExpr>,
    _1: Ws,
    _2: CharLiteral<b';'>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct ReturnStatementExpr {
    _0: Fws,
    pub expr: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct BreakStatement {
    #[literal = b"break"] _0: (),
    _1: Ws,
    _2: CharLiteral<b';'>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct VariableDeclarationStatement {
    #[literal = b"let"] _0: (),
    _1: Fws,
    pub declaration: VariableDeclaration,
    _2: Ws,
    _3: CharLiteral<b';'>,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AssignmentStatement {
    pub lhs: PrefixAssigneeExpr,
    _0: Ws,
    pub op: AssignmentOperator,
    _1: Ws,
    pub rhs: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PrefixAssigneeExpr {
    Postfix(PostfixAssigneeExpr),
    Prefix(PrefixAssigneeExprInner),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PrefixAssigneeExprInner {
    _0: CharLiteral<b'*'>,
    _1: Ws,
    pub expr: Box<PrefixAssigneeExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PostfixAssigneeExpr {
    pub variable: VariableReference,
    pub tail: Option<PostfixAssigneeExprTail>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PostfixAssigneeExprTail {
    Variable(VariableReference),
    Index(AssigneeIndexExpr),
    Member(AssigneeMemberExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AssigneeIndexExpr {
    _0: Ws,
    _1: CharLiteral<b'['>,
    _2: Ws,
    pub index_expr: Expr,
    _3: CharLiteral<b']'>,
    pub tail: Option<Box<PostfixAssigneeExprTail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AssigneeMemberExpr {
    _0: Ws,
    _1: CharLiteral<b'.'>,
    _2: Ws,
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
pub(crate) struct EqToken {
    #[literal = b"="] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct OrEqToken {
    #[literal = b"or="] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct XorEqToken {
    #[literal = b"xor="] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AndEqToken {
    #[literal = b"and="] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AddEqToken {
    #[literal = b"+="] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct SubEqToken {
    #[literal = b"-="] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MultEqToken {
    #[literal = b"*="] _0: (),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct DivEqToken {
    #[literal = b"/="] _0: (),
}



type ExprStatement = Expr;



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Expr {
    pub or_expr: OrExpr,
    _0: Ws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum OrExpr {
    Xor(XorExpr),
    Or(OrExprInner),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct OrExprInner {
    pub lhs: XorExpr,
    _0: Fws,
    #[literal = b"or"] _1: (),
    _2: Fws,
    pub rhs: Box<OrExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum XorExpr {
    And(AndExpr),
    Xor(XorExprInner),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct XorExprInner {
    pub lhs: AndExpr,
    _0: Fws,
    #[literal = b"xor"] _1: (),
    _2: Fws,
    pub rhs: Box<XorExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum AndExpr {
    Equality(EqualityExpr),
    And(AndExprInner),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AndExprInner {
    pub lhs: EqualityExpr,
    _0: Fws,
    #[literal = b"and"] _1: (),
    _2: Fws,
    pub rhs: Box<AndExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum EqualityExpr {
    Comp(CompExpr),
    Eq(EqExpr),
    Neq(NeqExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct EqExpr {
    pub lhs: CompExpr,
    _0: Ws,
    #[literal = b"=="] _1: (),
    _2: Ws,
    pub rhs: Box<EqualityExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct NeqExpr {
    pub lhs: CompExpr,
    _0: Ws,
    #[literal = b"!="] _1: (),
    _2: Ws,
    pub rhs: Box<EqualityExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum CompExpr {
    Additive(AdditiveExpr),
    Lt(LtExpr),
    Gt(GtExpr),
    Lte(LteExpr),
    Gte(GteExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LtExpr {
    pub lhs: AdditiveExpr,
    _0: Ws,
    _1: CharLiteral<b'<'>,
    _2: Ws,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct GtExpr {
    pub lhs: AdditiveExpr,
    _0: Ws,
    _1: CharLiteral<b'>'>,
    _2: Ws,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LteExpr {
    pub lhs: AdditiveExpr,
    _0: Ws,
    #[literal = b"<="] _1: (),
    _2: Ws,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct GteExpr {
    pub lhs: AdditiveExpr,
    _0: Ws,
    #[literal = b">="] _1: (),
    _2: Ws,
    pub rhs: Box<CompExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum AdditiveExpr {
    Multiplicative(MultiplicativeExpr),
    Add(AddExpr),
    Sub(SubExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct AddExpr {
    pub lhs: MultiplicativeExpr,
    _0: Ws,
    _1: CharLiteral<b'+'>,
    _2: Ws,
    pub rhs: Box<AdditiveExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct SubExpr {
    pub lhs: MultiplicativeExpr,
    _0: Ws,
    _1: CharLiteral<b'-'>,
    _2: Ws,
    pub rhs: Box<AdditiveExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum MultiplicativeExpr {
    Prefix(PrefixExpr),
    Mult(MultExpr),
    Div(DivExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MultExpr {
    pub lhs: PrefixExpr,
    _0: Ws,
    _1: CharLiteral<b'*'>,
    _2: Ws,
    pub rhs: Box<MultiplicativeExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct DivExpr {
    pub lhs: PrefixExpr,
    _0: Ws,
    _1: CharLiteral<b'/'>,
    _2: Ws,
    pub rhs: Box<MultiplicativeExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PrefixExpr {
    Postfix(PostfixExpr),
    Ref(RefExpr),
    Deref(DerefExpr),
    Minus(MinusExpr),
    Invert(InvertExpr),
    Not(NotExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct RefExpr {
    _0: CharLiteral<b'&'>,
    _1: Ws,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct DerefExpr {
    _0: CharLiteral<b'*'>,
    _1: Ws,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MinusExpr {
    _0: CharLiteral<b'-'>,
    _1: Ws,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct InvertExpr {
    _0: CharLiteral<b'~'>,
    _1: Ws,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct NotExpr {
    _0: CharLiteral<b'!'>,
    _1: Ws,
    pub expr: Box<PrefixExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PostfixExpr {
    pub inner: InnerExpr,
    pub tail: Option<PostfixExprTail>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum PostfixExprTail {
    Inner(InnerExpr),
    IndexExpr(IndexExpr),
    CallExpr(CallExpr),
    MemberExpr(MemberExpr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IndexExpr {
    _0: Ws,
    _1: CharLiteral<b'['>,
    _2: Ws,
    pub index_expr: Box<Expr>,
    _3: CharLiteral<b']'>,
    pub tail: Option<Box<PostfixExprTail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct CallExpr {
    _0: Ws,
    _1: CharLiteral<b'('>,
    _2: Ws,
    pub args: Option<FunctionArgumentList>,
    _3: CharLiteral<b')'>,
    pub tail: Option<Box<PostfixExprTail>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct MemberExpr {
    _0: Ws,
    _1: CharLiteral<b'.'>,
    _2: Ws,
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
    _0: CharLiteral<b'('>,
    pub expr: Box<Expr>,
    _1: CharLiteral<b')'>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitExpr {
    _0: CharLiteral<b'{'>,
    _1: Ws,
    pub list: Option<StructInitList>,
    _2: CharLiteral<b'}'>
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitList {
    pub entries: ZeroPlus<StructInitListPart>,
    pub last_entry: StructInitEntry,
    _0: Option<WsComma>,
    _1: Ws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitListPart {
    pub entry: StructInitEntry,
    _0: WsComma,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct StructInitEntry {
    pub name: VariableName,
    _0: Ws,
    _1: CharLiteral<b'='>,
    _2: Ws,
    pub expr: Box<Expr>,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct BuiltinFunctionCallExpr {
    _0: CharLiteral<b'@'>,
    pub function: FunctionReference,
    _1: Ws,
    _2: CharLiteral<b'('>,
    _3: Ws,
    pub args: Option<FunctionArgumentList>,
    _4: CharLiteral<b')'>,
}

type FunctionReference = VariableReference;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct FunctionArgumentList {
    pub args: ZeroPlus<FunctionArgumentListPart>,
    pub last_arg: FunctionArgument,
    _0: Option<WsComma>,
    _1: Ws,
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
    minus: Option<CharLiteral<b'-'>>,
    digits: Span<OnePlus<Numerical>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralIntBinary {
    #[literal = b"0b"] _0: (),
    digits: Span<OnePlus<BinaryOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralIntHexadecimal {
    #[literal = b"0x"] _0: (),
    digits: Span<OnePlus<HexadecimalOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralIntChar {
    _0: CharLiteral<b'\''>,
    char: CharChar,
    _1: CharLiteral<b'\''>,
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
    minus: Option<CharLiteral<b'-'>>,
    before_decimal: Span<OnePlus<Numerical>>,
    _0: CharLiteral<b'.'>,
    after_decimal: Span<OnePlus<Numerical>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralFloatScientific {
    mantissa_minus: Option<CharLiteral<b'-'>>,
    #[literal = b"0."] _0: (),
    mantissa: Span<OnePlus<Numerical>>,
    _1: CharLiteral<b'E'>,
    exponent_minus: Option<CharLiteral<b'-'>>,
    exponent: Span<OnePlus<Numerical>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralFloatBinary {
    mantissa_minus: Option<CharLiteral<b'-'>>,
    #[literal = b"0b"] _0: (),
    mantissa: Span<OnePlus<BinaryOrUnderscore>>,
    _1: CharLiteral<b'E'>,
    exponent_minus: Option<CharLiteral<b'-'>>,
    exponent: Span<OnePlus<BinaryOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralFloatHexadecimal {
    mantissa_minus: Option<CharLiteral<b'-'>>,
    #[literal = b"0x"] _0: (),
    mantissa: Span<OnePlus<HexadecimalOrUnderscore>>,
    _1: CharLiteral<b'E'>,
    exponent_minus: Option<CharLiteral<b'-'>>,
    exponent: Span<OnePlus<HexadecimalOrUnderscore>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct LiteralStringExpr {
    _0: CharLiteral<b'"'>,
    pub string: LiteralString,
    _1: CharLiteral<b'"'>,
}

type LiteralString = Span<ZeroPlus<StringChar>>;



type VisibilitySpecifier = Option<Public>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Public {
    #[literal = b"public"] _0: (),
    _1: Fws,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct VariableDeclaration {
    pub mutability_specifier: MutabilitySpecifier,
    pub variable_name: VariableName,
    _0: Fws,
    _1: CharLiteral<b':'>,
    _2: Ws,
    pub type_reference: TypeReference,
    _3: Fws,
    _4: CharLiteral<b'='>,
    _5: Ws,
    pub rhs: VariableDeclarationRhs,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum VariableDeclarationRhs {
    Undefined(Undefined),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Undefined {
    #[literal = b"undefined"] _0: (),
}

type MutabilitySpecifier = Option<Mutable>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct Mutable {
    #[literal = b"mut"] _0: (),
    _1: Fws,
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
    _0: CharLiteral<b'&'>,
    _1: Ws,
    type_reference: Box<TypeReference>,
}



type Identifier = Span<IdentifierInner>;

type Ws = Ignore<ZeroPlus<WhitespaceChar>>;

type Fws = Ignore<OnePlus<WhitespaceChar>>;

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct WsComma {
    _0: Ws,
    _1: CharLiteral<b','>,
}



#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct GlobalIdentifierPath {
    #[literal = b"root"]
    _0: (),
    pub path_parts: OnePlus<PathPart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct IdentifierPath {
    pub identifier: Identifier,
    pub path_parts: ZeroPlus<PathPart>,
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) struct PathPart {
    _0: Ws,
    _1: CharLiteral<b'.'>,
    _2: Ws,
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
}

#[derive(Clone, Debug, PartialEq, Eq, Parsable, Deserialize, Serialize)]
pub(crate) enum StringChar {
    EscapedDoubleQuote(EscapedDoubleQuote),
    SingleQuote(CharLiteral<b'\''>),
    EscapedChar(EscapedChar),
    NonEscapedChar(NonEscapedChar),
}

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
