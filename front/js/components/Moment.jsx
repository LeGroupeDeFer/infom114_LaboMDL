import React from 'react';
import formatDistance from 'date-fns/formatDistance';
import parseISO from 'date-fns/parseISO';
import fr from 'date-fns/locale/fr';

export default function Moment({ date, className }) {
  const parsed = parseISO(date);
  const now = new Date();
  const delta = formatDistance(parsed, now,{ locale: fr, addSuffix: true });
  return <span className={className}> {delta} </span>
}