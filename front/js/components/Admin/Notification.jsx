import React, { useState, useEffect } from 'react';

import Toast from 'react-bootstrap/Toast';

function Notification({text}) {
    const [show, setShow] = useState(false);
    
    //FIXME : When the XXXPage gets rerender (by adding a tag, or a role for instance), last toast is shown again
    useEffect(() =>{ 
        setShow(true);
    }, [text])

    return (         
        <Toast show={show} delay={3000} autohide onClose={() => setShow(!show)} style={{position: 'absolute', top:65, right:1, margin: '5px', zIndex:"100000"}}>
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
    );
};

export default Notification;