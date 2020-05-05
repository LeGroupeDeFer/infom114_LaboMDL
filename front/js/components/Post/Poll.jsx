import React, { useState } from 'react';
import { Button, Card, Row } from 'react-bootstrap';
import Form from 'react-bootstrap/Form';
import ProgressBar from 'react-bootstrap/ProgressBar';
import { FaRegCheckCircle } from 'react-icons/fa';

function Poll() {
  const options = ['Option 1', 'Option2'];
  const [voted, setVoted] = useState(false);

  return (
    <>
      {!voted && (
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
            <Button variant="primary" onClick={() => setVoted(true)}>
              Voter
            </Button>
          </Card.Body>
        </Card>
      )}

      {voted && (
        <Card onClick={(e) => e.preventDefault} className="poll mb-2">
          <Card.Header>
            <span className="ml-">12 votes</span>
          </Card.Header>
          <Card.Body>
            <div>
              <ProgressBar
                animated
                now={70}
                className="mb-2"
                label={
                  <span className="progress-value">
                    <span className="mr-5">35</span>
                    <span>Option 1</span>
                    <FaRegCheckCircle size={20} className="ml-1 opt-selected" />
                  </span>
                }
              />
              <ProgressBar
                animated
                now={2}
                className="mb-2"
                label={
                  <span className="progress-value">
                    <span className="mr-5">1</span>
                    <span>Option 2 avec looong label</span>
                  </span>
                }
              />

              <ProgressBar
                animated
                now={28}
                className="mb-2"
                label={
                  <span className="progress-value">
                    <span className="mr-5">14</span>
                    <span>Option 3</span>
                  </span>
                }
              />
            </div>
          </Card.Body>
        </Card>
      )}
    </>
  );
}

export default Poll;
