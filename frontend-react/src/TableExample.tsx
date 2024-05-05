import { AgGridReact } from 'ag-grid-react';
import "ag-grid-community/styles/ag-grid.css";
import "ag-grid-community/styles/ag-theme-quartz.css";
import {useState} from "react";

// reference https://www.ag-grid.com/react-data-grid/getting-started/
export const TableExample = () => {
    // Row Data: The data to be displayed.
    const [rowData] = useState([
        {make: "Tesla", model: "Model Y", price: 64950, electric: true},
        {make: "Ford", model: "F-Series", price: 33850, electric: false},
        {make: "Toyota", model: "Corolla", price: 29600, electric: false},
        {make: "Toyota", model: "Corolla", price: 29600, electric: false},
        {make: "Toyota", model: "Corolla", price: 29600, electric: false},
        {make: "Toyota", model: "Corolla", price: 29600, electric: false},
    ]);

    // Column Definitions: Defines the columns to be displayed.
    const [colDefs ] = useState<any>([
        {field: "make", width: 150},
        {field: "model", flex: 1},
        {field: "price",flex: 1},
        {field: "electric",flex: 1}
    ]);

    return (
        <div
            className="ag-theme-quartz" // applying the grid theme
        >
            <AgGridReact
                rowData={rowData}
                columnDefs={colDefs}
                domLayout='autoHeight'

            />
        </div>
    )
}