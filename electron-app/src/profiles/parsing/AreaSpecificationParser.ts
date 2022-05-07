import { IAreaSpecification, MonitorDistance } from "../Profile";
import { Token, TokenType, TokenWithLocation } from "./Lexer";

// DE-PARSING ---------------------------------------------

export function composeAreaSpecifications(specs: IAreaSpecification[]): string {
    let outputs = specs.map(spec => {
        let result = "";
        if (spec.selector === undefined) {
            result += "*";
        } else {
            result += `${spec.selector.width}x${spec.selector.height}`;
        }
        result += " {\n";

        result += `   x: ${composeMonitorDistance(spec.x)};\n`;
        result += `   y: ${composeMonitorDistance(spec.y)};\n`;
        result += `   width: ${composeMonitorDistance(spec.width)};\n`;
        result += `   height: ${composeMonitorDistance(spec.height)};\n`;

        result += "}";

        return result;
    });

    return outputs.join("\n\n");
}

function composeMonitorDistance(dist: MonitorDistance) {
    if ('px' in dist) {
        return dist.px.toString() + "px";
    } else {
        return dist.percentage.toString() + "%";
    }
}

// PARSING ---------------------------------------------

export class AreaSpecificationsParser {
    tokens: TokenWithLocation[] = []

    parse(tokens: TokenWithLocation[]): IAreaSpecification[] {
        this.tokens = tokens.reverse();
        const result: IAreaSpecification[] = [];

        while(this.tokens.length > 0) {
            result.push(this.areaSpecification());
        }

        return result;
    }

    areaSpecification(): IAreaSpecification {
        const selector = this.selector();
        let next = this.tokens.pop();
        if (next?.type !== TokenType.BlockOpen) {
            throw this.throw(TokenType.BlockOpen, next);
        }
        const fields = new Map<string, MonitorDistance>();
        next = this.tokens[this.tokens.length - 1];
        while (next.type != TokenType.BlockClose) {
            const fieldName = this.field();
            if (fields.has(fieldName)) {
                throw `'${fieldName}' was defined twice in this block ${this.formatLocation(next)}`;
            }
            next = this.tokens.pop();
            if (next?.type !== TokenType.FieldEnd) {
                throw this.throw(TokenType.FieldEnd, next);
            }
            const value = this.monitorDistance();
            fields.set(fieldName, value);
            next = this.tokens.pop();
            if (next?.type !== TokenType.StatementEnd) {
                throw this.throw(TokenType.StatementEnd, next);
            }
            next = this.tokens[this.tokens.length - 1];
        }
        if (next?.type !== TokenType.BlockClose) {
            throw this.throw(TokenType.BlockClose, next);
        }
        this.tokens.pop();
        ["width", "height", "x", "y"].forEach(field => {
            if (!fields.has(field)) {
                throw `Area is missing '${field}' field ${next ? this.formatLocation(next) : "EOF"}`;
            }
        });
        return {
            selector,
            width: fields.get("width")!,
            height: fields.get("height")!,
            x: fields.get("x")!,
            y: fields.get("y")!,
        }
    }

    monitorDistance(): MonitorDistance {
        const value = this.number();
        const next = this.tokens.pop();
        if (next?.type === TokenType.PercentageSign) {
            if ((value < 0 || value > 100)) {
                throw `Value out of bounds, must be 0-100 ${this.formatLocation(next)}`;
            }
            return { percentage: value };
        } else if (next?.type === TokenType.Literal) {
            if (next.value === "px") {
                return { px: value };
            }
            throw `Unexpected '${next.value}', only 'px' and '%' units are supported ${this.formatLocation(next)}`;
        }
        throw `Only 'px' and '%' units are supported ${next ? this.formatLocation(next) : "EOF"}`;
    }

    number(): number {
        let next = this.tokens.pop();
        if (next?.type !== TokenType.Integer) {
            throw this.throw(TokenType.Integer, next);
        }
        let val = next.value;
        if (this.tokens[this.tokens.length - 1].type === TokenType.DecimalPoint) {
            this.tokens.pop();
            val += ".";
            let next = this.tokens.pop();
            if (next?.type !== TokenType.Integer) {
                throw this.throw(TokenType.Integer, next);
            }
            val += next.value;
        }

        return Number(val);
    }

    field(): "width" | "height" | "x" | "y" {
        const next = this.tokens.pop();
        if (next?.type !== TokenType.Literal) {
            throw this.throw(TokenType.StatementEnd, next);
        }
        if (next.value === "width" || next.value === "height" || next.value === "x" || next.value === "y") {
            return next.value;
        }
        throw `Field name must be one of x, y, width or height ${this.formatLocation(next)}`;
    }

    selector(): undefined | { width: number, height: number } {
        let next = this.tokens.pop();
        if (next?.type === TokenType.Wildcard) {
            return undefined;
        } else if (next?.type === TokenType.Integer) {
            const width = Number(next.value);
            next = this.tokens.pop();
            if (next?.type !== TokenType.Literal || next.value !== "x") {
                throw this.throw("'x'", next);
            }
            next = this.tokens.pop();
            if (next?.type !== TokenType.Integer) {
                throw this.throw(TokenType.Integer, next);
            }
            const height = Number(next.value);
            return { height, width };
        }
        throw this.throw("'*' or monitor dimensions", next);
    }

    throw(expected: string, actual: TokenWithLocation | undefined) {
        if (actual) {
            return `Expected ${expected}, got ${actual.type} ${this.formatLocation(actual)}.`;
        }
        return `Expected ${expected}, got EOF.`;
    }

    formatLocation(token: TokenWithLocation) {
        return `(line ${token.line}, col ${token.col})`;
    }

}