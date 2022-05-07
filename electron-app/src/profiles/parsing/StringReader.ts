
export class StringReader {
    private _line = 1;
    private _col = 1;

    constructor(private str: string) {
    }

    trim() {
        const match = this.str.search(/\S/);
        if (match > 0) {
            this.consume(match);
        }
    }

    peek(numChars: number): string {
        // if (this.str.length < numChars) {
        //     throw "Internal parser error";
        // }
        return this.str.substr(0, numChars);
    }

    consume(numChars: number): string {
        const consumed = this.peek(numChars);

        this.str = this.str.substr(numChars);

        if (consumed.indexOf("\n") >= 0) {
            this._line += consumed.split("\n").length - 1;
            this._col = consumed.length - consumed.lastIndexOf("\n") + 1;
        } else {
            this._col += consumed.length;
        }

        return consumed;
    }

    isEOF() {
        return this.str === "";
    }

    get line() { return this._line; }
    get col() { return this._col; }
}