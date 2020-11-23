/**
 * Single Sign-On API (Public)
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

export class RequestAuditReadSeek {
    'id'?: number;
    'limit': number;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "id",
            "baseName": "id",
            "type": "number"
        },
        {
            "name": "limit",
            "baseName": "limit",
            "type": "number"
        }    ];

    static getAttributeTypeMap() {
        return RequestAuditReadSeek.attributeTypeMap;
    }
}

