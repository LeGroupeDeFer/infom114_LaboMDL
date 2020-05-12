import React from 'react';
import formatDistance from 'date-fns/formatDistance';
import formatRelative from 'date-fns/formatRelative';
import parseISO from 'date-fns/parseISO';
import fr from 'date-fns/locale/fr';
import { capitalize } from 'unanimity/lib';

export default function Moment({ date, className, relative, capitalized }) {
  const parsed = parseISO(date);
  const now = new Date();

  let delta;
  if (relative)
    delta = formatRelative(parsed, now, { locale: fr });
  else
    delta = formatDistance(parsed, now,{ locale: fr, addSuffix: true });
  return <span className={className}> {capitalize ? capitalize(delta) : delta} </span>
}