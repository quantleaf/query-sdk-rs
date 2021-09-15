use crate::models::code_language::LanguageCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type EnumDomain = HashMap<String, SimpleDescription>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SimpleDomain {
    Date,
    Number,
    Text,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]

pub enum Domain {
    SimpleDomain(SimpleDomain),
    EnumDomain(EnumDomain),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StringOrStringVec {
    String(String),
    StringVec(),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SimpleDescription {
    LanguageMap(HashMap<LanguageCode, StringOrStringVec>),
    String(String),
    StringVec(Vec<String>),
}
/*
pub fn unwrapDescription(simpleDescription:SimpleDescription) =>
{
    const description:{[key in LanguageCode]?:String[]} = {};
    if(Array.isArray(simpleDescription))
    {
        // No language code
        description[LanguageCode.ANY] = simpleDescription as string[];
    }
    else if((typeof simpleDescription) === 'string')
    {
        description[LanguageCode.ANY] = [simpleDescription as string];

    }
    else
    {
        const keys = Object.keys(simpleDescription);
        for(const key of keys)
        {
            let value = simpleDescription[key];
            if(typeof value === 'string')
            {
                value = [value];
            }
            description[(key as LanguageCode)] = value;
        }
    }
    return description;
}

pub const unwrapDomain = (domain:StandardDomain|EnumDomain):StandardDomain|EnumDomain =>
{
    if(!domain)
        return null;
    if(!DomainUtils.isEnumDomain(domain))
        return domain;

    const unwrappedEnumType:EnumDomain = {};

    if(Array.isArray(domain))
    {
        for (let i = 0; i < domain.length; i++) {
            unwrappedEnumType[domain[i]] =
            {
                [LanguageCode.ANY]: [domain[i]]
            }

        }
    }
    else
    {
        for(const key of Object.keys(domain as EnumDomain))
        {
            unwrappedEnumType[key] = unwrapDescription(domain[key]);
        }
    }

    return unwrappedEnumType;
}*/

/**
 * Unwraps inplace and returns
 * @param schema
 * @return unwrapped schema
 */
/* pub const unwrapSchema = (schema:Schema) => {
    const newFields = [];
    schema.fields.forEach((field)=>
    {
        newFields.push(Field.from(field.key,field.description,field.domain));
    });
    schema.fields = newFields;
    schema.name = {
        key:schema.name.key,
        description:unwrapDescription(schema.name.description)
    }
    return schema;
}*/

/*
pub struct CustomTypeRefence
{
    key:String;
}*/
/*pub struct Typed
{

    domain?:StandardDomain|EnumDomain|ExternalDomain;
}*/

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct KeyWithDescriptions
//, TypeDeclarationOrTyped
{
    pub key: String,
    pub description: SimpleDescription,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct QueryOperations {
    pub negative: bool,
    pub nesting: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Schema {
    pub name: KeyWithDescriptions,
    pub fields: Vec<Field>,
    pub operations: QueryOperations,
}
/*pub struct FieldFormattedKey extends Field
{
    formattedKey:String;
}*/

/*
pub struct ExternalDomain  {

    header?:String,
    url:String
};

static isEnumDomain(domain:StandardDomain|EnumDomain|ExternalDomain)
{
  /*  if(DomainUtils.isExternalDomain(domain))
        return false;*/
    if(domain == undefined)
    return false;
    return typeof domain != 'string';
}

static isStandardDomain(domain:StandardDomain|EnumDomain|ExternalDomain)
{
    if(domain == undefined)
    return false;
    return typeof domain == 'string';
}*/

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Field {
    // The key of the field
    pub key: String,

    // The  natural language representation
    pub description: SimpleDescription,

    // The domain for the value
    pub domain: Domain,
}

/*
public static from(key:String, description:SimpleDescription, domain:StandardDomain|EnumDomain): Field
    {
        return {
            description: unwrapDescription(description),
            domain: unwrapDomain(domain),
            key:key
        }
    }

    public static clone(clone:Field)
    {
        let descriptionClone:{[key in LanguageCode]?:String[]} = null;
        if(clone.description)
        {
            descriptionClone = {};
            Object.keys(clone.description).forEach((k)=>
            {
                descriptionClone[k] = Info.clone(clone.description[k]);
            })
        }
        const ret:Field = {
            description: descriptionClone,
            key: clone.key,
            domain: clone.domain
        }
        return ret;
    }
    */
