'use strict';

export interface IRect {
    width: number;
    height: number;
    x: number;
    y: number;
}

export interface IProfile {
    regex: string;
    area: IRect;
}
