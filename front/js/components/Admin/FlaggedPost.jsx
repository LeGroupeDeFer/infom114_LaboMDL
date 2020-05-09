import React, { useState, useEffect } from 'react';

import Card from 'react-bootstrap/Card';
import Badge from 'react-bootstrap/Badge';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

import Moment from 'react-moment';
import { FaEllipsisH, FaEyeSlash, FaFacebookSquare, FaFlag, FaLock, FaTag, FaTrashAlt } from 'react-icons/fa';

const FlaggedPost = ({ post, count_flag, reasons}) => {

    const { author, type, id, createdAt, title, tags, content } = post;

    return (
        <Card>
            <Card.Header>
                <Container className="p-0">
                    <Row>
                        <Col className="col-sm-10">
                            <h5 className="ml-1">
                                <Badge className={`post-${type} mr-1`}>{type}</Badge>
                                <span className="mr-1">{title}</span>

                                <span className="text-muted title-part2">
                                    <a href="#" className="text-dark mx-1">
                                        <span>{author.firstname}</span>
                                        <span className="ml-1">{author.lastname}</span>
                                    </a>
                                    <span>-</span>
                                    <Moment locale="fr" fromNow className="ml-1">{createdAt}</Moment>
                                </span>
                            </h5>
                        </Col>
                        <Col className="mr-auto">
                            <span>A été signalé {count_flag} fois </span><FaFlag />
                        </Col>
                    </Row>
                </Container>
            </Card.Header>

            <Card.Body className="post-body p-0 expand-preview">
                <div className="d-flex expand-preview">
                    <div className="px-3 pb-3 pt-2 w-100">
                        <Card.Text>
                            <div className="mb-1">
                                {tags.map(tag => (
                                    <a href="#" key={tag.id} className="mr-2 tag">
                                        <FaTag className="mr-1" />
                                        <span>{tag.label}</span>
                                    </a>
                                ))}
                            </div>

                            {content}

                        </Card.Text>
                    </div>
                </div>
            </Card.Body>
            <Card.Footer>
                {
                    reasons.length > 1 ? <b>Raisons du signalement :</b> : <b>Raison du signalement</b> 
                }
                <br /><hr />
                {
                    reasons.map((reason, i) => {
                        return (i + 1 === reasons.length ? <> {reason}<br /></> : <>{reason}<br /><hr /></>)
                    })
                }
            </Card.Footer>

        </Card>
    );

};

export default FlaggedPost;