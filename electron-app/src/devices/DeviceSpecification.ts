'use strict';
import { WledData } from './WledSettings';
import { QmkData } from './QmkSettings';

export interface IDeviceSpecification {
    name: string;
    numberOfLeds: number;
    gamma: number;
    colorTemp: number;
    saturationAdjustment: number;
    valueAdjustment: number;
    useAudio: boolean;
    preferredMonitor: number;
    type: number | null;
    wledData: WledData | null;
    qmkData: QmkData | null
}

export enum DeviceTypes {
  WLED,
  QMK,
}
