'use strict';
import { IWledData } from './WledSettings';
import { IQmkData } from './QmkSettings';

export interface IDeviceSpecification {
    name: string;
    numberOfLeds: number;
    gamma: number;
    colorTemp: number;
    saturationAdjustment: number;
    valueAdjustment: number;
    audioAmount: number;
    preferredMonitor: number;
    type: number | null;
    wledData: IWledData | null;
    qmkData: IQmkData | null
}

export enum DeviceTypes {
  WLED,
  QMK,
}
