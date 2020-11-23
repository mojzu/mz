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

export class ResponseOpenidConfiguration {
    'authorizationEndpoint': string;
    'issuer': string;
    'tokenEndpoint': string;
    'tokenEndpointAuthMethodsSupported': Array<string>;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "authorizationEndpoint",
            "baseName": "authorization_endpoint",
            "type": "string"
        },
        {
            "name": "issuer",
            "baseName": "issuer",
            "type": "string"
        },
        {
            "name": "tokenEndpoint",
            "baseName": "token_endpoint",
            "type": "string"
        },
        {
            "name": "tokenEndpointAuthMethodsSupported",
            "baseName": "token_endpoint_auth_methods_supported",
            "type": "Array<string>"
        }    ];

    static getAttributeTypeMap() {
        return ResponseOpenidConfiguration.attributeTypeMap;
    }
}

