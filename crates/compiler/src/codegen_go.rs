use std::collections::HashMap;

const BUFFER_SIZE: usize = 65536;

pub struct GoCodegen {
    type_map: HashMap<&'static str, &'static str>,
    indent: usize,
    code: String,
}

impl GoCodegen {
    pub fn new() -> Self {
        let mut type_map = HashMap::new();
        type_map.insert("Int", "int");
        type_map.insert("String", "string");

        Self {
            type_map,
            indent: 0,
            code: String::with_capacity(BUFFER_SIZE),
        }
    }

    fn map_type<'a>(&self, interop_type: &'a str) -> &'a str {
        self.type_map.get(interop_type).unwrap_or(&interop_type)
    }

    fn push_indent(&mut self) {
        for _ in 0..self.indent {
            self.code.push('\t');
        }
    }

    pub fn gen_code(&mut self, decls: &[ast::Decl<'_>]) -> &str {
        for decl in decls {
            match decl {
                ast::Decl::Func(func) => {
                    self.gen_func(func);
                }
            }
        }

        &self.code
    }

    fn gen_func(&mut self, func: &ast::Func) {
        // Write function signature
        self.code.push_str("func ");
        self.code.push_str(func.name.as_ref());

        // Write function args
        self.code.push('(');
        for (idx, arg) in func.args.iter().enumerate() {
            self.code.push_str(arg.name.as_ref());
            self.code.push(' ');
            self.code.push_str(self.map_type(arg.arg_type.as_ref()));

            if idx < func.args.len() - 1 {
                self.code.push_str(", ");
            }
        }
        self.code.push_str(") ");

        // Write function body
        match &func.body {
            ast::FuncBody::Block(Some(type_), block) => {
                self.code.push_str(self.map_type(type_.as_ref()));
                self.code.push(' ');
                self.gen_block(block, true);
            }
            ast::FuncBody::Block(None, block) => {
                self.gen_block(block, true);
            }
            ast::FuncBody::Expr(expr) => {
                self.gen_single_stmt_block(expr, true);
            }
        }
    }

    fn gen_field(&mut self, field: &ast::Field<'_>) {
        self.gen_simple_expr(&field.target);
        self.code.push('.');
        self.code.push_str(field.field.as_ref());
    }

    fn gen_call(&mut self, call: &ast::Call<'_>) {
        self.gen_simple_expr(&call.target);

        self.code.push('(');

        // TODO: Named args may be out of order - we ignore this for now
        for (idx, arg) in call.args.iter().enumerate() {
            self.gen_simple_expr(&arg.expr);

            if idx < call.args.len() - 1 {
                self.code.push_str(", ");
            }
        }

        self.code.push(')');
    }

    fn gen_bool_cond(&mut self, bool_cond: &ast::BoolCond<'_>) {
        match bool_cond {
            ast::BoolCond::Not(expr) => {
                self.code.push_str("!");
                self.gen_simple_expr(expr);
            }
            ast::BoolCond::And(lhs, rhs) => {
                self.gen_simple_expr(lhs);
                self.code.push_str(" && ");
                self.gen_simple_expr(rhs);
            }
            ast::BoolCond::Or(lhs, rhs) => {
                self.gen_simple_expr(lhs);
                self.code.push_str(" || ");
                self.gen_simple_expr(rhs);
            }
        }
    }

    // In Go, if is always a statement, so more handling is needed here
    fn gen_if_then_else(&mut self, if_then_else: &ast::IfThenElse<'_>) {
        self.code.push_str("if ");
        self.gen_simple_expr(&if_then_else.cond);
        self.code.push(' ');
        self.gen_single_stmt_block(&if_then_else.then, false);
        self.code.push_str(" else ");
        self.gen_single_stmt_block(&if_then_else.else_, false);
        self.code.push('\n');
    }

    // In Go, if is always a statement, so more handling is needed here
    fn gen_if(&mut self, if_: &ast::If<'_>) {
        self.code.push_str("if ");
        self.gen_simple_expr(&if_.cond);
        self.code.push(' ');
        self.gen_block(&if_.then_body, false);

        if let Some(else_body) = &if_.else_body {
            self.code.push_str(" else ");

            match else_body {
                ast::ElseBody::If(else_if) => {
                    self.gen_if(else_if);
                }
                ast::ElseBody::Block(block) => {
                    self.gen_block(block, false);
                }
            }
        }
    }

    fn gen_simple_expr(&mut self, expr: &ast::SimpleExpr<'_>) {
        match expr {
            ast::SimpleExpr::Ident(ident) => {
                self.code.push_str(ident.as_ref());
            }
            ast::SimpleExpr::IntLit(int_lit) => {
                self.code.push_str(&format!("{}", int_lit.value));
            }
            ast::SimpleExpr::StringLit(string_lit) => {
                self.code.push_str(&string_lit.unparsed);
            }
            ast::SimpleExpr::CharLit(char_lit) => {
                self.code.push_str(&char_lit.unparsed);
            }
            ast::SimpleExpr::BoolLit(bool_lit) => {
                self.code.push_str(&format!("{}", bool_lit.0));
            }
            ast::SimpleExpr::Field(field) => {
                self.gen_field(field);
            }
            ast::SimpleExpr::Call(call) => {
                self.gen_call(call);
            }
            ast::SimpleExpr::IfThenElse(if_then_else) => {
                self.gen_if_then_else(if_then_else);
            }
            ast::SimpleExpr::BoolCond(bool_cond) => {
                self.gen_bool_cond(bool_cond);
            }
            ast::SimpleExpr::Expr(expr) => {
                self.gen_expr(expr);
            }
        }
    }

    fn gen_expr(&mut self, expr: &ast::Expr<'_>) {
        match expr {
            ast::Expr::If(if_) => {
                self.gen_if(if_);
            }
            ast::Expr::Simple(simple_expr) => {
                self.gen_simple_expr(simple_expr);
            }
        }
    }

    fn gen_single_stmt_block(&mut self, expr: &ast::SimpleExpr<'_>, func_block: bool) {
        self.code.push_str("{\n");

        self.indent += 1;
        self.push_indent();

        if func_block {
            // TODO: This is a hack to handle the last statement in a block for now
            self.code.push_str("return ");
        }
        self.gen_simple_expr(expr);
        self.code.push('\n');

        self.indent -= 1;
        self.push_indent();
        self.code.push('}');
    }

    fn gen_block(&mut self, block: &ast::Block<'_>, func_block: bool) {
        self.code.push_str("{\n");
        self.indent += 1;

        for (idx, stmt_or_expr) in block.stmt_or_exprs.iter().enumerate() {
            self.push_indent();
            // This is a hack to handle the last statement in a block
            // and it won't work for more complex cases
            if func_block && idx == block.stmt_or_exprs.len() - 1 {
                self.code.push_str("return ");
            }

            match stmt_or_expr {
                ast::StmtOrExpr::Func(func) => {
                    self.gen_func(func);
                }
                ast::StmtOrExpr::Expr(expr) => {
                    self.gen_expr(expr);
                }
            }

            self.code.push('\n');
        }

        self.indent -= 1;
        self.push_indent();
        self.code.push('}');
    }
}
