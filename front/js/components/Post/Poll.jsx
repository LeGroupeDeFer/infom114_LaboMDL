import React, { useState } from 'react';
import {
  Button,
  Card,
  Row,
  Col,
  OverlayTrigger,
  Tooltip,
} from 'react-bootstrap';

import Form from 'react-bootstrap/Form';
import ProgressBar from 'react-bootstrap/ProgressBar';
import CheckCircle from '../../icons/check-circle-regular.svg';
import { useAuth } from 'unanimity/context';

function Poll({ postId, answers, userAnswer, onPollVote }) {
  const { user } = useAuth();
  const isLogged = !!user;
  const [userVote, setUserVote] = useState(
    userAnswer == null ? null : userAnswer.id
  );
  const [optionSelected, setOptionSelected] = useState(null);
  let toltalVote = 0;
  answers.forEach((a) => (toltalVote += a.count));

  function vote() {
    onPollVote(postId, optionSelected);
    setUserVote(optionSelected);
  }

  return (
    <>
      <Card onClick={(e) => e.preventDefault} className="poll my-2 w-100">
        <Card.Header>
          <span className="ml-3">
            {toltalVote} {toltalVote > 1 ? ' votes' : ' vote'}
          </span>
        </Card.Header>
        <Card.Body>
          {userVote == null && (
            <>
              {(answers || []).map((opt, index) => {
                return (
                  <Form.Check
                    type="radio"
                    label={opt.answer}
                    id={`opt-${opt.id}`}
                    className="mb-3"
                    name="poll-options"
                    onChange={() => setOptionSelected(opt.id)}
                    key={index}
                  />
                );
              })}

              {isLogged ? (
                <Button
                  variant="primary"
                  onClick={() => vote()}
                  disabled={optionSelected == null}
                >
                  Voter
                </Button>
              ) : (
                <OverlayTrigger
                  placement="right"
                  overlay={
                    <Tooltip>
                      Il faut être authentifié pour pouvoir partiticiper à un
                      sondage
                    </Tooltip>
                  }
                >
                  <Button variant="primary" disabled>
                    Voter
                  </Button>
                </OverlayTrigger>
              )}
            </>
          )}

          {userVote != null && (
            <>
              {answers.map((opt, index) => {
                return (
                  <ProgressBar
                    animated
                    key={index}
                    now={opt.count == 0 ? 0.5 : (opt.count * 100) / toltalVote}
                    className="mb-2"
                    label={
                      <div className="progress-value">
                        <Row>
                          <Col xs={1} className="text-right">
                            {opt.count}
                          </Col>
                          <Col xs={11} className="text-left">
                            {opt.answer}
                            {userVote == opt.id && (
                              <CheckCircle className="ml-2 opt-selected" />
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
