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
      <img
        id="faq-img"
        src="https://axanar.com/wp-content/uploads/2016/01/faq-icon-1024x361.png"
        className="mx-auto d-block"
      ></img>
      <hr />
      <Accordion>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="0">
              A quoi sert la plateforme Unanimity ?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="0">
            <Card.Body>Unanimity est une plateforme participative intégrée à l'UNamur. Son but est de rassembler 
            les membres de l'université qu'ils soient membres du personnel éducatif ou étudiant afin de:
            <ul class="list-group mt-3">
                <li class="list-group-item"><b>Mieux informer</b></li>
                <li class="list-group-item"><b>Débattre</b></li>       
                <li class="list-group-item"><b>Soumettre des idées</b></li>
                <li class="list-group-item"><b>Avoir un impact sur les décisions de l’Université au niveau étudiant, facultaire et institutionnel</b></li>
            </ul>
            </Card.Body>
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
                <b>- Les sondages</b> : Similaires aux idées mais permettent aux membres de voter différentes propositions 
              </p>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="2">
              Mon idée/vote peut-elle/il aboutir à quelque chose de concret ?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="2">
            <Card.Body>
              <p><b>Oui !</b> Certains modérateurs ont pour tâche de juger les idées 
              et permettre un suivi sur les plus pertinents. Ce suivi pourra alors
              aboutir à une acceptation ou un rejet. Dans le premier cas, de réelles démarches seront mises en places 
              afin d'arriver à un aboutissement concret sur le campus universitaire.
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
