import type { Author } from "./Author";
import type { User } from "./User";

export interface Link {
    module: string,
    object: string,
}

export interface Links {
    inbound: Link[],
    outbound: Link[],
}

export interface CustomFields extends IHash {}

export interface Object {
    id: number,
    level: string,
    header: string,
    content: string,
    author: User | string, 
    isActive: boolean,
    isNormative: boolean,
    isRequirement: boolean,
    createdAt: Date,
    updatedAt: Date,
    deletedAt: Date | null,
    customFiels: CustomFields | null,
}

export interface ObjectView {
    object: Object,
    links: Link[] = [],
    isDraft: boolean = false,
    hasChanges: boolean = false,
}

/*
1 Introdução
 1.1 Objetivos
 1.2 Escopo
 1.3 Disposições
2 Terminologia
2.a Esta é a seção de terminologias, definições e etc 
 2.1 Abreviaturas
 2.1-a ADC
 2.1-b DAC
 2.1-c RBT
3 Referências Normativas
3.a Como este projeto é clasificado blá blá bá
3.1 Normas Brasileiras
3.1.1 NBR 7240-1 Termos e Definições
3.1.1-a Esta norma é importante
3.1.1-b Esta norma é importante
3.2 

[
    {
        level: "1",
        header: "Introdução",
        path: "",
        children: [
            {
                level: "1.1",
                header: "Objetivos",
                path: ""
                children: [],
            },
            {
                level: "1.2",
                header: "Escopo",
                path: ""
                children: [],
            },            
            {
                level: "1.3",
                header: "Disposições",
                path: ""
                children: [],
            },            
        ]
    },
    {
        level: "2",
        header: "Introdução",
        path: "",
        children: [
            {
                level: "2.1",
                header: "Abreviaturas",
                path: ""
                children: [],
            }           
        ]
    },
    {
        level: "3",
        header: "Referências Normativas",
        path: "",
        children: [
            {
                level: "3.1",
                header: "Normas Brasileiras",
                path: ""
                children: [
                    {
                        level: "3.1.1",
                        header: "NBR 7240-1 Termos e Definições",
                        path: ""
                        children: [],
                    } 
                ],
            }           
        ]
    },
]
 

*/