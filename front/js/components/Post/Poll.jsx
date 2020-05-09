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
  const [userVote, setUserVote] = useState(null);
  const [optionSelected, setOptionSelected] = useState(null);

  function vote(option) {
    // Api call then
    setUserVote(option);
  }

  return (
    <>
      <Card onClick={(e) => e.preventDefault} className="poll mb-2">
        <Card.Header>
          <span className="ml-">50 votes</span>
        </Card.Header>
        <Card.Body>
          {userVote == null && (
            <>
              {options.map((opt, index) => {
                return (
                  <Form.Check
                    type="radio"
                    label={opt}
                    id={`opt-${index + 1}`}
                    className="mb-3"
                    name="poll-options"
                    onChange={() => setOptionSelected(opt)}
                  />
                );
              })}
              <Button
                variant="primary"
                onClick={() => vote(optionSelected)}
                disabled={optionSelected == null}
              >
                Voter
              </Button>
            </>
          )}

          {userVote != null && (
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
                            {opt.option}
                            {userVote == opt.option && (
                              <FaRegCheckCircle
                                size={20}
                                className="ml-2 opt-selected"
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
