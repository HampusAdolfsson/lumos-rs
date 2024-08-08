'use strict';
import { IWledData } from './WledSettings';
import { IQmkData } from './QmkSettings';
import { ISerialData } from './SerialSettings';

export interface IDeviceSpecification {
    name: string;
    numberOfLeds: number;
    samplingType: number;
    gamma: number;
    colorTemp: number;
    saturationAdjustment: number;
    valueAdjustment: number;
    audioAmount: number;
    fallbackColor: [number, number, number],
    type: number | null;
    wledData: IWledData | null;
    qmkData: IQmkData | null;
    serialData: ISerialData | null;
}

export enum SamplingTypes {
  Horizonal,
  Vertical,
}

export enum DeviceTypes {
  WLED,
  QMK,
  Serial,
}
