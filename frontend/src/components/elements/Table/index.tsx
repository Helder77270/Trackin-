import { match } from "ts-pattern";
import classes from "./classes.module.scss";

type IProps = {
    tableHeads: TableHeads[];
    tableData: TableData[];
    isLoading?: boolean;
};

type TableHeads = {
    label: string;
    cellType: ECellType;
};

type TableData = {
    [key: string]: string | number;
};

export enum ECellType {
    EURO = "euro",
    PERCENTAGE = "percentage",
    TEXT = "text",
    PATATE = "patate",
}
export default function Table(props: IProps) {
    const { isLoading = false } = props;

    return (
        <table className={classes.root}>
            <thead>
                <tr>
                    {props.tableHeads.map((head, index) => (
                        <th key={index}>{head.label}</th>
                    ))}
                </tr>
            </thead>
            {!isLoading && (
                <tbody>
                    {props.tableData.map((data, index) => (
                        <tr key={index}>
                            {props.tableHeads.map((head, index) => (
                                <td key={index}>
                                    {renderCell(
                                        data[head.label],
                                        head.cellType
                                    )}
                                </td>
                            ))}
                        </tr>
                    ))}
                </tbody>
            )}
            {isLoading && (
                <tbody>
                    <tr>
                        <td colSpan={props.tableHeads.length}>Loading...</td>
                    </tr>
                </tbody>
            )}
        </table>
    );
}
// npm i ts-pattern
// npm i typescript@latest
function renderCell(text: string | number, cellType: ECellType) {
    return match(cellType)
        .with(ECellType.EURO, () => {
            if (typeof text === "string") {
                return text;
            }

            return (
                text
                    .toFixed(2)
                    .replace(/\d(?=(\d{3})+\.)/g, "$& ")
                    .replace(".", ",") + " €"
            );
        })
        .with(ECellType.PERCENTAGE, () => {
            return `${text} %`;
        })
        .with(ECellType.TEXT, () => {
            return text;
        })
        .with(ECellType.PATATE, () => {
            return `🥔 ${text} 🥔`;
        })
        .exhaustive();
}
