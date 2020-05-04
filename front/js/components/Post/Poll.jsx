import React from 'react';
import { Button, Card, Row } from 'react-bootstrap';
import Form from 'react-bootstrap/Form';

function Poll() {
  const options = ['Option 1', 'Option2'];

  return (
    <>
      <Card onClick={(e) => e.preventDefault} className="poll mb-2">
        <Card.Header>
          <span className="ml-">12 votes</span>
        </Card.Header>
        <Card.Body>
          <Card.Text>
            {options.map((opt, index) => {
              return (
                <Form.Check
                  type="radio"
                  label={opt}
                  id={`opt-${index + 1}`}
                  className="mb-3"
                  name="poll-options"
                />
              );
            })}
          </Card.Text>
          <Button variant="primary">Voter</Button>
        </Card.Body>
      </Card>
    </>
  );
}

export default Poll;
