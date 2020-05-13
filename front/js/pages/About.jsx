import React from 'react';
import { Accordion, Card, Container, Button } from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';

export default function About(props) {
  return (
    <Container className="py-5">
      <hr />

      {/* Numbers */}
      <Card className="mb-5">
        <Card.Body>
          <div className="text-center">
            <p>
              Unanimity est une plateforme collaborative de discussions et de
              prises de décision relative à l'UNamur.
            </p>

            <p>
              Vous en avez assez des longues réunions et des chaînes de mails
              infinies qui ne semblent jamais aboutir à une décision ou à un
              résultat de groupe ? On dirait que vous avez besoin de UNanimity :
              une plateforme de collaboration qui transforme radicalement la
              façon dont les instances facultaires prennent des décisions,
              discutent des idées et collaborent. Dites adieu aux réunions
              fastidieuses et aux courriels qui encombrent votre boîte de
              réception et essayez gratuitement les outils de prise de décision
              de UNanimity !
            </p>
          </div>
        </Card.Body>
      </Card>

      {/* FAQ */}
      <h2 className="text-center text-dark faq-header mb-5">
        <Icon icon="lightbulb" className="mr-4" />
        <span>FAQ</span>
        <hr />
      </h2>

      <Accordion>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="0">
              A quoi sert la plateforme Unanimity ?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="0">
            <Card.Body>
              Unanimity est une plateforme participative intégrée à l'UNamur.
              Son but est de rassembler les membres de l'université qu'ils
              soient membres du personnel éducatif ou étudiant afin de:
              <ul className="list-group mt-3">
                <li className="list-group-item">
                  <b>Mieux informer</b>
                </li>
                <li className="list-group-item">
                  <b>Débattre</b>
                </li>
                <li className="list-group-item">
                  <b>Soumettre des idées</b>
                </li>
                <li className="list-group-item">
                  <b>
                    Avoir un impact sur les décisions de l’Université au niveau
                    étudiant, facultaire et institutionnel
                  </b>
                </li>
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
                <b>- Les sondages</b> : Similaires aux idées mais permettent aux
                membres de voter différentes propositions
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
              <p>
                <b>Oui !</b> Certains modérateurs ont pour tâche de juger les
                idées et permettre un suivi sur les plus pertinentes. Ce suivi
                pourra alors aboutir à une acceptation ou un rejet. Dans le
                premier cas, de réelles démarches seront mises en places afin
                d'arriver à un aboutissement concret sur le campus
                universitaire.
              </p>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="3">
              Comment fonctionne l'algorithme de rang des publications ?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="3">
            <Card.Body>
              <p>
                L'algorithme suivant est utilisé pour donner une meilleure
                visibilité aux publications les plus pertinentes :
              </p>

              <code>// pseudocode</code>
              <br />
              <code>base = sqrt(2)</code>
              <br />
              <code>epoch = 01/01/2020</code>
              <br />
              <code>
                rank = log_sqrt(2) (votes) + (epoch - publication_date)
              </code>
              <p>
                Avec
                <ul>
                  <li>
                    la variable <code>base</code> qui a été choisie de concert
                    avec notre product owner.
                  </li>
                  <li>
                    la variable <code>epoch</code> est utilisée comme point de
                    départ de l'application
                  </li>
                </ul>
              </p>
              <p>
                De cette façon, les votes permettent de faire monter
                temporairement le ranking d'une publication. En effet, après un
                certain temps, les nouvelles publications auront un{' '}
                <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ">
                  <i>rank</i>
                </a>{' '}
                équivalent aux vielles publications qui ont un haut score.
              </p>
              <p>
                Avec les paramètres actuels, un post qui a 100 votes positifs
                arrivera à un <i>rank</i> équivalent à un post de 0 vote créé 15
                jours plus tard.
              </p>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
        <Card>
          <Card.Header>
            <Accordion.Toggle as={Button} variant="link" eventKey="4">
              Une publication ayant un score négatif possède un meilleur <i>rank</i> que la mienne. Comment est-ce possible?
            </Accordion.Toggle>
          </Card.Header>
          <Accordion.Collapse eventKey="4">
            <Card.Body>
              <p>
                Le <i>ranking</i> de l'application se base surtout sur l'activité de la publication.
                Cela signifie que toutes les interactions avec une publication vont augmenter la pertinence qu'elle a d'être "poussée vers le haut".
              </p>
              <p>
                Si je juge qu'une publication n'est pas intéressante, la meilleure façon de réagir est de ne pas intéragir avec celle-ci.
              </p>
              <p>
                Par contre il m'est possible de faire remarquer à un modérateur qu'une publication ne respecte pas la politique d'utilisation de l'appliation via l'action "signaler" .
              </p>
            </Card.Body>
          </Accordion.Collapse>
        </Card>
      </Accordion>
    </Container>
  );
}
