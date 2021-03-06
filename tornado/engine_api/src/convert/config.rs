use serde_json::Error;
use tornado_engine_api_dto::config::{
    ActionDto, ConstraintDto, ExtractorDto, ExtractorRegexDto, FilterDto, MatcherConfigDto,
    OperatorDto, RuleDto,
};
use tornado_engine_matcher::config::filter::Filter;
use tornado_engine_matcher::config::rule::{
    Action, Constraint, Extractor, ExtractorRegex, Operator, Rule,
};
use tornado_engine_matcher::config::MatcherConfig;

pub fn matcher_config_into_dto(config: MatcherConfig) -> Result<MatcherConfigDto, Error> {
    Ok(match config {
        MatcherConfig::Ruleset { name, rules } => MatcherConfigDto::Ruleset {
            name,
            rules: rules.into_iter().map(rule_into_dto).collect::<Result<Vec<_>, _>>()?,
        },
        MatcherConfig::Filter { name, filter, nodes } => MatcherConfigDto::Filter {
            name,
            filter: filter_into_dto(filter)?,
            nodes: nodes.into_iter().map(matcher_config_into_dto).collect::<Result<Vec<_>, _>>()?,
        },
    })
}

pub fn filter_into_dto(filter: Filter) -> Result<FilterDto, Error> {
    Ok(FilterDto {
        description: filter.description,
        filter: filter.filter.map(operator_into_dto).transpose()?,
        active: filter.active,
    })
}

pub fn rule_into_dto(rule: Rule) -> Result<RuleDto, Error> {
    Ok(RuleDto {
        active: rule.active,
        actions: rule.actions.into_iter().map(action_into_dto).collect::<Result<Vec<_>, _>>()?,
        constraint: constraint_into_dto(rule.constraint)?,
        description: rule.description,
        do_continue: rule.do_continue,
        name: rule.name,
    })
}

pub fn action_into_dto(action: Action) -> Result<ActionDto, Error> {
    Ok(ActionDto { id: action.id, payload: serde_json::to_value(action.payload)? })
}

pub fn constraint_into_dto(constraint: Constraint) -> Result<ConstraintDto, Error> {
    Ok(ConstraintDto {
        where_operator: constraint.where_operator.map(operator_into_dto).transpose()?,
        with: constraint
            .with
            .into_iter()
            .map(|(key, value)| (key, extractor_into_dto(value)))
            .collect(),
    })
}

pub fn operator_into_dto(operator: Operator) -> Result<OperatorDto, Error> {
    let result = match operator {
        Operator::And { operators } => OperatorDto::And {
            operators: operators
                .into_iter()
                .map(operator_into_dto)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Operator::Or { operators } => OperatorDto::Or {
            operators: operators
                .into_iter()
                .map(operator_into_dto)
                .collect::<Result<Vec<_>, _>>()?,
        },
        Operator::Contain { first, second } => OperatorDto::Contain {
            first: serde_json::to_value(&first)?,
            second: serde_json::to_value(&second)?,
        },
        Operator::Equal { first, second } => OperatorDto::Equal {
            first: serde_json::to_value(&first)?,
            second: serde_json::to_value(&second)?,
        },
        Operator::GreaterEqualThan { first, second } => OperatorDto::GreaterEqualThan {
            first: serde_json::to_value(&first)?,
            second: serde_json::to_value(&second)?,
        },
        Operator::GreaterThan { first, second } => OperatorDto::GreaterThan {
            first: serde_json::to_value(&first)?,
            second: serde_json::to_value(&second)?,
        },
        Operator::LessEqualThan { first, second } => OperatorDto::LessEqualThan {
            first: serde_json::to_value(&first)?,
            second: serde_json::to_value(&second)?,
        },
        Operator::LessThan { first, second } => OperatorDto::LessThan {
            first: serde_json::to_value(&first)?,
            second: serde_json::to_value(&second)?,
        },
        Operator::Regex { regex, target } => OperatorDto::Regex { regex, target },
    };
    Ok(result)
}

pub fn extractor_into_dto(extractor: Extractor) -> ExtractorDto {
    ExtractorDto { from: extractor.from, regex: extractor_regex_into_dto(extractor.regex) }
}

pub fn extractor_regex_into_dto(extractor_regex: ExtractorRegex) -> ExtractorRegexDto {
    ExtractorRegexDto {
        group_match_idx: extractor_regex.group_match_idx,
        regex: extractor_regex.regex,
    }
}
