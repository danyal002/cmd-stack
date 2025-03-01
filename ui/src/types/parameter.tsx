export enum ParameterType {
    String = "String",
    Int = "Int",
    Boolean = "Boolean",
    Blank = "Blank"
};

export type Parameter = {
    type: ParameterType,
    data: {
        min: Number,
        max: Number
    }
}
