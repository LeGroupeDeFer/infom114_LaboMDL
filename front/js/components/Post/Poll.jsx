import React, { useState } from 'react';
import { Button, Card, Row, Col } from 'react-bootstrap';
import Form from 'react-bootstrap/Form';
import ProgressBar from 'react-bootstrap/ProgressBar';
import { FaRegCheckCircle } from 'react-icons/fa';

function Poll() {
  const options = ['Option 1', 'Option 2', 'Option 3'];
  const [voted, setVoted] = useState(false);

  return (
    <>
      {!voted && (
        <Card onClick={(e) => e.preventDefault} className="poll mb-2">
          <Card.Header>
            <span className="ml-">50 votes</span>
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
            <span className="ml-">50 votes</span>
          </Card.Header>
          <Card.Body>
            <div>
              <ProgressBar
                animated
                now={70}
                className="mb-2"
                label={
                  <div className="progress-value">
                    <Row>
                      <Col xs={1} className="text-right">
                        35
                      </Col>
                      <Col xs={11} className="text-left">
                        Option 1
                        <FaRegCheckCircle
                          size={20}
                          className="ml-1 opt-selected"
                        />
                      </Col>
                    </Row>
                  </div>
                }
              />
              <ProgressBar
                animated
                now={2}
                className="mb-2"
                label={
                  <div className="progress-value">
                    <Row>
                      <Col xs={1} className="text-right">
                        1
                      </Col>
                      <Col xs={11} className="text-left">
                        Option 2
                      </Col>
                    </Row>
                  </div>
                }
              />

              <ProgressBar
                animated
                now={28}
                className="mb-2"
                label={
                  <div className="progress-value">
                    <Row>
                      <Col xs={1} className="text-right">
                        14
                      </Col>
                      <Col xs={11} className="text-left">
                        Option 3
                      </Col>
                    </Row>
                  </div>
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
