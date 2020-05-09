import React, { useState, useEffect } from 'react';

import Card from 'react-bootstrap/Card';
import Badge from 'react-bootstrap/Badge';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

import Moment from 'react-moment';
import { FaEllipsisH, FaEyeSlash, FaFacebookSquare, FaFlag, FaLock, FaTag, FaTrashAlt } from 'react-icons/fa';

const FlaggedPost = ({ post }) => {

    const { author, type, id, createdAt, title, tags, content } = post;

    return (
        <Card>
            <Card.Header>
                <Container className="p-0">
                    <Row>
                        <Col className="expand-preview">
                            <h5 className="ml-1 expand-preview">
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

                        <div className="post-footer mb-2">

                        </div>
                    </div>
                </div>
            </Card.Body>

        </Card>
    );

};

export default FlaggedPost;