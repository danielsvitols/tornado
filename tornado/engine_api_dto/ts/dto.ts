


/* tslint:disable */

/* WARNING: this file was automatically generated at compile time */
/* DO NOT CHANGE IT MANUALLY */

/* ------------ */
/* custom types */
/* ------------ */

export type Value = any;


/* -------------- */
/* 'config' types */
/* -------------- */

export type ActionDto = { id: string; payload: Value };

export type ConstraintDto = { WHERE: OperatorDto | null; WITH: { [key: string]: ExtractorDto } };

export type ExtractorDto = { from: string; regex: ExtractorRegexDto };

export type ExtractorRegexDto = { match: string; group_match_idx: number };

export type FilterDto = { description: string; active: boolean; filter: OperatorDto | null };

export type MatcherConfigDto = 
 | {     type: "Filter"; name: string; filter: FilterDto; nodes:     MatcherConfigDto [] } 
 | { type: "Ruleset"; name: string; rules: RuleDto [] };

export type OperatorDto = 
 | { type: "AND"; operators: OperatorDto [] } 
 | { type: "OR"; operators: OperatorDto [] } 
 | { type: "contain"; first: Value; second: Value } 
 | { type: "equal"; first: Value; second: Value } 
 | { type: "ge"; first: Value; second: Value } 
 | { type: "gt"; first: Value; second: Value } 
 | { type: "le"; first: Value; second: Value } 
 | { type: "lt"; first: Value; second: Value } 
 | { type: "regex"; regex: string; target: string };

export type RuleDto = {     name: string; description: string; continue: boolean; active:     boolean; constraint: ConstraintDto; actions: ActionDto [] };


/* ------------- */
/* 'event' types */
/* ------------- */

export type EventDto = { type: string; created_ms: number; payload: { [key: string]: Value } };

export enum ProcessType { Full = "Full", SkipActions = "SkipActions" };

export type ProcessedEventDto = { event: EventDto; result: ProcessedNodeDto };

export type ProcessedFilterDto = { status: ProcessedFilterStatusDto };

export enum ProcessedFilterStatusDto { Matched = "Matched", NotMatched = "NotMatched", Inactive = "Inactive" };

export type ProcessedNodeDto = 
 | {     type: "Filter"; name: string; filter: ProcessedFilterDto; nodes:     ProcessedNodeDto [] } 
 | { type: "Ruleset"; name: string; rules: ProcessedRulesDto };

export type ProcessedRuleDto = {     name: string; status: ProcessedRuleStatusDto; actions: ActionDto [];     message: string | null };

export type ProcessedRulesDto = { rules: ProcessedRuleDto []; extracted_vars: { [key: string]: Value } };

export enum ProcessedRuleStatusDto {     Matched = "Matched", PartiallyMatched = "PartiallyMatched", NotMatched =     "NotMatched", NotProcessed = "NotProcessed" };

export type SendEventRequestDto = { process_type: ProcessType; event: EventDto };