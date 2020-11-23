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
import { ResponseAccessManyData } from './responseAccessManyData';

export class ResponseAccessMany {
    'data': Array<ResponseAccessManyData>;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "data",
            "baseName": "data",
            "type": "Array<ResponseAccessManyData>"
        }    ];

    static getAttributeTypeMap() {
        return ResponseAccessMany.attributeTypeMap;
    }
}

