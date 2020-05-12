import React from 'react';
import { Accordion, Card, Container, Button } from 'react-bootstrap';

export default function About(props) {
  return (
    <Container className="py-5">
      <h4 className="text-center"> Unanimity en chiffres</h4>
      <hr />
      <Card className="mb-4">
        <Card.Body>
          <div className="text-center">
            <p>
              Unanimity c'est <b>3</b> utilisateurs, <b>25</b> idées mises en
              place et blabla..
            </p>
          </div>
        </Card.Body>
      </Card>
      <br />
      <br />
      <h4 className="text-center"> FAQ</h4>
      <hr />
      <Accordion>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="0">
              A quoi sert la plateforme Unanimity ?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="0">
            <Card.Body>A rien wallah</Card.Body>
          </Accordion.Collapse>
        </Card>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="1">
              Quel sont les différents types de publications ?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="1">
            <Card.Body>
              <p> Il existe 3 catégories de publications.</p>
              <p>
                <b> - Les informations</b> : qui sont utilisés par
                l'administration et qui servent à informer les membres de la
                plateforme.
              </p>
              <p>
                <b>- Les idées</b> : N'importe quel utilisateur enregistré sur
                la plateforme peut soumettre une idée. Son idée est alors
                analysée par l'administration qui peut l'accepter ou la
                rejetter.
              </p>
              <p>
                <b>- Les sondages</b> : Blablabla..
              </p>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="2">
              Comment fonctionne l'algorithme de rang des publications ?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="2">
            <Card.Body>
              <p>C'est magique</p>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
      </Accordion>
    </Container>
  );
}
