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
import { RequestAuditReadSeek } from './requestAuditReadSeek';

export class RequestAuditRead {
    'auditType'?: Array<string>;
    'id'?: Array<number>;
    'seek': RequestAuditReadSeek;
    'subject'?: Array<string>;
    'userId'?: Array<string>;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "auditType",
            "baseName": "audit_type",
            "type": "Array<string>"
        },
        {
            "name": "id",
            "baseName": "id",
            "type": "Array<number>"
        },
        {
            "name": "seek",
            "baseName": "seek",
            "type": "RequestAuditReadSeek"
        },
        {
            "name": "subject",
            "baseName": "subject",
            "type": "Array<string>"
        },
        {
            "name": "userId",
            "baseName": "user_id",
            "type": "Array<string>"
        }    ];

    static getAttributeTypeMap() {
        return RequestAuditRead.attributeTypeMap;
    }
}

