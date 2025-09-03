import React from 'react'
import AllClients from "./lib/AllClients";
import {BrowserRouter, Routes, Route, Link } from 'react-router-dom'
import ClientPage from "./lib/ClientPage";

export default function App() {

    return (
        <BrowserRouter>
            <nav>
                <Link to = "/clients">All Clients</Link> <br/>
            </nav>
            <Routes>
                <Route path = "/clients" element = {<AllClients/>}/>
                <Route path = "/client/:clientName" element = {<ClientPage/>}/>
            </Routes>
        </BrowserRouter>
    )
}
