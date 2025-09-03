import {useParams} from "react-router-dom";


export default function ClientPage() {
    const { clientName } = useParams();
    console.log(useParams());

    return (
        <p>
            {clientName}
        </p>
    )
}