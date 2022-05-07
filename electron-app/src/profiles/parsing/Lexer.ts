import { StringReader } from "./StringReader";

export enum TokenType {
    StatementEnd = ";",
    BlockOpen = "{",
    BlockClose = "}",
    FieldEnd = ":",
    DecimalPoint = ".",
    PercentageSign = "%",
    Wildcard = "*",
    Literal = "Identifier",
    Integer = "Number",
    Unknown = "Unknown",
}

interface TokenNode<T extends TokenType> {
  type: T;
}

interface TokenValueNode<T extends TokenType> extends TokenNode<T> {
  value: string;
}

export type Token =
  TokenNode<TokenType.StatementEnd> |
  TokenNode<TokenType.BlockOpen> |
  TokenNode<TokenType.BlockClose> |
  TokenNode<TokenType.FieldEnd> |
  TokenNode<TokenType.DecimalPoint> |
  TokenNode<TokenType.PercentageSign> |
  TokenNode<TokenType.Wildcard> |
  TokenValueNode<TokenType.Literal> |
  TokenValueNode<TokenType.Integer> |
  TokenValueNode<TokenType.Unknown>;

const tokenMap: Array<{ key: string, token: Token }> = [
    { key: TokenType.StatementEnd, token: { type: TokenType.StatementEnd } },
    { key: TokenType.BlockOpen, token: { type: TokenType.BlockOpen } },
    { key: TokenType.BlockClose, token: { type: TokenType.BlockClose } },
    { key: TokenType.FieldEnd, token: { type: TokenType.FieldEnd } },
    { key: TokenType.DecimalPoint, token: { type: TokenType.DecimalPoint } },
    { key: TokenType.PercentageSign, token: { type: TokenType.PercentageSign } },
    { key: TokenType.Wildcard, token: { type: TokenType.Wildcard } },
];

export interface Location {
    line: number;
    col: number;
}

export type TokenWithLocation = Token & Location;

export function tokenize(reader: StringReader) {
    const out: TokenWithLocation[] = [];

    let i = 0;
    while(!reader.isEOF() && i++ < 100) {
        // @ts-ignore
        if (reader.peek(1) === " " || reader.peek(1) === "\n") {
            reader.consume(1);
            continue;
        }

        let matched = false;

        for(const { key, token } of tokenMap) {
            if (lookahead(reader, key)) {
                out.push({
                    ...token,
                    line: reader.line,
                    col: reader.col,
                });
                reader.consume(key.length);
                matched = true;
            }
        }
        if (matched) continue;

        const intVal = consumeInteger(reader);
        if (intVal !== "") {
            out.push({
                type: TokenType.Integer,
                value: intVal,
                line: reader.line,
                col: reader.col,
            });
            continue;
        }

        const strVal = consumeString(reader);
        if (strVal !== "") {
            out.push({
                type: TokenType.Literal,
                value: strVal,
                line: reader.line,
                col: reader.col,
            });
            continue;
        }
        out.push({
            type: TokenType.Unknown,
            value: reader.consume(1),
            line: reader.line,
            col: reader.col,
        });
    }

    return out;
}

function consumeInteger(str: StringReader): string {
    let out = "";
    while (!str.isEOF() && str.peek(1).match(/\d/)) {
        out += str.consume(1);
    }
    return out;
}

function consumeString(str: StringReader): string {
    let out = "";
    while(str.peek(1).match(/[a-zA-Z]/)) {
        out += str.consume(1);
    }
    return out;
}

function lookahead(str: StringReader, target: string): boolean {
    return str.peek(target.length) === target;
}