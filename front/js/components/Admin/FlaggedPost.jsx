import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';

import { Card, Badge, Container, Row, Col } from 'react-bootstrap/';
import Moment from 'react-moment';

import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faLink, faFlag, faTag } from '@fortawesome/free-solid-svg-icons';

const FlaggedPost = ({ post, countFlag, reasons, onHide }) => {
  const { author, kind, id, createdAt, title, tags, content } = post;

  return (
    <Card>
      <Card.Header>
        <Container className="p-0">
          <Row>
            <Col>
              <h5 className="ml-1">
                <Badge className={`post-${kind} mr-1`}>{kind}</Badge>
                <span className="mr-1">{title}</span>

                <span className="text-muted title-part2">
                  <a href="#" className="text-dark mx-1">
                    <span>{author.firstname}</span>
                    <span className="ml-1">{author.lastname}</span>
                  </a>
                  <span>-</span>
                  <Moment locale="fr" fromNow className="ml-1">
                    {createdAt}
                  </Moment>
                </span>
              </h5>
            </Col>
          </Row>
        </Container>
      </Card.Header>

      <Card.Body className="post-body p-0 expand-preview">
        <div className="d-flex expand-preview">
          <div className="px-3 pb-3 pt-2 w-100">
            <Card.Text>
              <div className="mb-1">
                {tags.map((tag) => (
                  <a key={tag} className="mr-2 tag">
                    <Icon icon={faTag} className="mr-1" />
                    <span>{tag}</span>
                  </a>
                ))}
              </div>

              <span>{content}</span>
            </Card.Text>
            <div className="post-footer mb-2">
              <Link
                to={`/detail/${id}`}
                className="post-footer-btn mr-2"
                href="#"
              >
                <Icon icon={faLink} />
                <span> - Voir le post</span>
              </Link>
              <br />
              <hr />
              <span className="text-muted">
                <Icon icon={faFlag} />
                <span className="text-muted">
                  {' '}
                  - A été signalé {countFlag} fois{' '}
                </span>
              </span>
            </div>
          </div>
        </div>
      </Card.Body>
      <Card.Footer>
        {reasons.length > 1 ? (
          <b>Raisons du signalement :</b>
        ) : (
          <b>Raison du signalement</b>
        )}
        <br />
        <hr />
        {reasons.map((reason, i) => {
          return i + 1 === reasons.length ? (
            <>
              {' '}
              {reason}
              <br />
            </>
          ) : (
            <>
              {reason}
              <br />
              <hr />
            </>
          );
        })}
      </Card.Footer>
    </Card>
  );
};

export default FlaggedPost;
