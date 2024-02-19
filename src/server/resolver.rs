use crate::cli_params::CliParam;

use super::message::{answer::Answer, question::Question};

pub enum Resolver {
    Default,
    Custom(String),
}

impl From<&[CliParam]> for Resolver {
    fn from(params: &[CliParam]) -> Self {
        let resolver_param = params
            .iter()
            .find(|param| matches!(param, CliParam::Resolver(_)));
        match resolver_param {
            Some(CliParam::Resolver(value)) => Resolver::Custom(value.clone()),
            None => Resolver::Default,
        }
    }
}

pub fn resolve_questions(questions: &[Question], resolver: &Resolver) -> Vec<Answer> {
    match resolver {
        Resolver::Default => default_resolver(questions),
        Resolver::Custom(resolver) => custom_resolver(questions, resolver),
    }
}

fn default_resolver(questions: &[Question]) -> Vec<Answer> {
    questions.iter().map(Answer::for_question).collect()
}

fn custom_resolver(questions: &[Question], _resolver: &String) -> Vec<Answer> {
    questions.iter().map(Answer::for_question).collect()
}
