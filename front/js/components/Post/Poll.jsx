import React, { useState } from 'react';
import { Button, Card, Row, Col } from 'react-bootstrap';
import Form from 'react-bootstrap/Form';
import ProgressBar from 'react-bootstrap/ProgressBar';
import { FaRegCheckCircle } from 'react-icons/fa';

function Poll() {
  const options = ['Option 1', 'Option 2', 'Option 3'];
  const optionVote = [
    { option: 'Option 1', vote: '14' },
    { option: 'Option 2', vote: '1' },
    { option: 'Option 3', vote: '35' },
  ];

  const toltalVote = 50;
  const [vote, setVote] = useState(null);
  const [radioSelected, setRadioSelected] = useState(false);

  function vote(option) {
    // Api call then
    setVote(option);
  }

  return (
    <>
      <Card onClick={(e) => e.preventDefault} className="poll mb-2">
        <Card.Header>
          <span className="ml-">50 votes</span>
        </Card.Header>
        <Card.Body>
          {vote == null && (
            <>
              {options.map((opt, index) => {
                return (
                  <Form.Check
                    type="radio"
                    label={opt}
                    id={`opt-${index + 1}`}
                    className="mb-3"
                    name="poll-options"
                    onChange={() => setRadioSelected(true)}
                  />
                );
              })}
              <Button
                variant="primary"
                onClick={() => vote(opt)}
                disabled={radioSelected}
              >
                Voter
              </Button>
            </>
          )}

          {vote != null && (
            <>
              {optionVote.map((opt, index) => {
                return (
                  <ProgressBar
                    animated
                    key={index}
                    now={(opt.vote * 100) / toltalVote}
                    className="mb-2"
                    label={
                      <div className="progress-value">
                        <Row>
                          <Col xs={1} className="text-right">
                            {opt.vote}
                          </Col>
                          <Col xs={11} className="text-left">
                            {opt.label}
                            {vote == opt.label && (
                              <FaRegCheckCircle
                                size={20}
                                className="ml-1 opt-selected"
                              />
                            )}
                          </Col>
                        </Row>
                      </div>
                    }
                  />
                );
              })}
            </>
          )}
        </Card.Body>
      </Card>
      )}
    </>
  );
}

export default Poll;

{
  /* Object.keys(optionVote).map((obj, idex) => {
  return (<ProgressBar
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
</>)} */
}
