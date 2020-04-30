import React, { useState, useEffect } from 'react';

import Toast from 'react-bootstrap/Toast';

function TagToast({text}) {
    const [show, setShow] = useState(true);
    
    useEffect(() =>{ 
        setShow(true)
    }, [text])

    return (         
        <Toast show={show} delay={3000} autohide onClose={() => setShow(!show)} style={{position: 'absolute',top: 0,right: 0, margin: '5px'}}>
            <Toast.Header>
                <img
                src="holder.js/20x20?text=%20"
                className="rounded mr-2"
                alt=""
                />
                <strong className="mr-auto">Attention !</strong>
            </Toast.Header>
            <Toast.Body>{text}</Toast.Body>
        </Toast>
    )
}

export default TagToast