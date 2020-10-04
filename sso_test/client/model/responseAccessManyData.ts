/**
 * Single Sign-On API
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v2
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';

export class ResponseAccessManyData {
    'clientId': string;
    'createdAt': Date;
    'enable': boolean;
    'scope': string;
    '_static': boolean;
    'updatedAt': Date;
    'userId': string;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "clientId",
            "baseName": "client_id",
            "type": "string"
        },
        {
            "name": "createdAt",
            "baseName": "created_at",
            "type": "Date"
        },
        {
            "name": "enable",
            "baseName": "enable",
            "type": "boolean"
        },
        {
            "name": "scope",
            "baseName": "scope",
            "type": "string"
        },
        {
            "name": "_static",
            "baseName": "static",
            "type": "boolean"
        },
        {
            "name": "updatedAt",
            "baseName": "updated_at",
            "type": "Date"
        },
        {
            "name": "userId",
            "baseName": "user_id",
            "type": "string"
        }    ];

    static getAttributeTypeMap() {
        return ResponseAccessManyData.attributeTypeMap;
    }
}

