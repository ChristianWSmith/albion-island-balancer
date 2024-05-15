use linprog::{
    Model,
    Objective,
    Summand,
    Operator,
    Var
};

fn main() {
    let mut model = Model::new("My LP", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    vars.push(model.reg_var(2.0));
    model.reg_constr(vec![Summand(1.0, &vars[0])], Operator::Le, 10.0);
    model.optimize();
    println!("{}", model);
}
