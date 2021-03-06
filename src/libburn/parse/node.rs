use std::vec::Vec;
use mem::raw::Raw;
use lang::identifier::Identifier;
use vm::analysis::annotation;

pub struct Root {
	pub statements: Vec<Box<Statement>>,
	pub frame: annotation::Frame,
}

// STATEMENTS //////////////////////////////////////////////////////////////////////////////////////

pub enum Statement {
	
	Use {
		pub path: Vec<Identifier>,
		pub annotation: annotation::Use,
	},
	
	ExpressionStatement {
		pub expression: Box<Expression>,
	},
	
	Assignment {
		pub lvalue: Box<Lvalue>,
		pub rvalue: Box<Expression>,
	},
	
	Let {
		pub variable_offset: uint,
		pub variable_name: Identifier,
		pub annotation: Raw<annotation::Variable>,
		pub default: Option<Box<Expression>>,
	},
	
	Print {
		pub expression: Box<Expression>,
	},
	
	Return {
		pub expression: Option<Box<Expression>>,
	},
	
	Throw {
		pub expression: Box<Expression>,
	},
	
	If {
		pub test: Box<Expression>,
		pub block: Vec<Box<Statement>>,
		pub else_if_clauses: Vec<Box<ElseIf>>,
		pub else_clause: Option<Box<Else>>,
	},
	
	Try {
		pub block: Vec<Box<Statement>>,
		pub catch_clauses: Vec<Box<Catch>>,
		pub else_clause: Option<Box<Else>>,
		pub finally_clause: Option<Box<Finally>>,
	},
	
	While {
		pub test: Box<Expression>,
		pub block: Vec<Box<Statement>>,
		pub else_clause: Option<Box<Else>>,
	},
}

pub struct ElseIf {
	pub test: Box<Expression>,
	pub block: Vec<Box<Statement>>,
}

pub struct Else {
	pub block: Vec<Box<Statement>>,
}

pub struct Catch {
	pub type_: Option<Box<Expression>>,
	pub variable_name: Identifier,
	pub variable: Raw<annotation::Variable>,
	pub block: Vec<Box<Statement>>,
}

pub struct Finally {
	pub block: Vec<Box<Statement>>,
}

// EXPRESSIONS /////////////////////////////////////////////////////////////////////////////////////

pub enum Expression {
	
	Function {
		pub parameters: Vec<FunctionParameter>,
		pub frame: annotation::Frame,
		pub block: Vec<Box<Statement>>,
	},
	
	And {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Or {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Not {
		pub expression: Box<Expression>,
	},
	
	Is {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Eq {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Neq {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Lt {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Gt {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	LtEq {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	GtEq {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	
	Union {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	
	Addition {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Subtraction {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Multiplication {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	Division {
		pub left: Box<Expression>,
		pub right: Box<Expression>,
	},
	
	DotAccess {
		pub expression: Box<Expression>,
		pub name: Identifier,
	},
	
	ItemAccess {
		pub expression: Box<Expression>,
		pub key_expression: Box<Expression>,
	},
	
	Call {
		pub expression: Box<Expression>,
		pub arguments: Vec<Box<Expression>>,
	},
	
	Variable {
		pub name: Identifier,
		pub annotation: Raw<annotation::Variable>,
		pub source_offset: uint,
	},
	
	Name {
		pub identifier: Identifier,
		pub annotation: annotation::Name,
	},
	
	String {
		pub value: ::std::string::String,
	},
	Integer {
		pub value: i64,
	},
	Float {
		pub value: f64,
	},
	Boolean {
		pub value: bool,
	},
	Nothing,
}

pub struct FunctionParameter {
	pub type_: Option<Box<Expression>>,
	pub default: Option<Box<Expression>>,
	pub variable_name: Identifier,
	pub variable: Raw<annotation::Variable>,
}

// LVALUES /////////////////////////////////////////////////////////////////////////////////////////

pub enum Lvalue {
	
	VariableLvalue {
		pub name: Identifier,
		pub annotation: Raw<annotation::Variable>,
		pub source_offset: uint,
	},
	
	DotAccessLvalue {
		pub expression: Box<Expression>,
		pub name: Identifier,
	},
}
