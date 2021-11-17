'use strict';

export interface IRect {
    width: number;
    height: number;
    x: number;
    y: number;
}

export interface IProfile {
    id: number;
    regex: string;
    area: IRect;
    priority: number | undefined;
}

export interface IProfileCategory {
    name: string;
    profiles: IProfile[];
    priority: number;
}