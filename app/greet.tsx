'use client'

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { DataGrid } from '@mui/x-data-grid';
import { Container } from '@mui/material';


export default function Greet() {
    const [get_dataframe, setDataframe] = useState('');

    const rows = [
        { id: 1, name: 'John Doe', age: 25 },
        { id: 2, name: 'Jane Smith', age: 30 },
        { id: 3, name: 'Albert Johnson', age: 35 },
        // 他のデータ...
    ];
      
    const columns = [
        { field: 'id', headerName: 'ID', width: 90 },
        { field: 'name', headerName: 'Name', width: 150 },
        { field: 'age', headerName: 'Age', width: 110 },
        // 他のカラム...
    ];
      
    useEffect(() => {
        invoke<string>('get_dataframe', { name: 'Next.js'})
            .then(result => setDataframe(result))
            .catch(console.error)
    }, [])

    return (
        <div>{JSON.stringify(get_dataframe, null, 2)}
        <Container style={{ height: 400, width: '100%' }}>
            <DataGrid rows={rows} columns={columns} pageSize={5} checkboxSelection />
        </Container>
        </div>
    );
}



// export default function Greet() {
//     const [greeting, setGreeting] = useState('');

//     useEffect(() => {
//         invoke<string>('greet', { name: 'Next.js'})
//             .then(result => setGreeting(result))
//             .catch(console.error)
//     }, [])

//     return <div>{greeting}</div>
// }
