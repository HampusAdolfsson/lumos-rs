'use strict';

export interface IDeviceSpecification {
    ipAddress: string;
    numberOfLeds: number;
    saturationAdjustment: number;
    blurRadius: number;
    flipHorizontally: boolean;
}
