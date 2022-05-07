'use strict';

// export interface IMonitorDistance {

// }

export type MonitorDistance = { px: number } | { percentage: number };

export interface IAreaSpecification {
    selector: undefined | { width: number, height: number };
    width: MonitorDistance;
    height: MonitorDistance;
    x: MonitorDistance;
    y: MonitorDistance;
}

export interface IProfile {
    id: number;
    regex: string;
    areas: IAreaSpecification[];
    priority: number | undefined;
}

export interface IProfileCategory {
    name: string;
    profiles: IProfile[];
    priority: number;
}