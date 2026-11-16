// Paper UI: screens, HUD and menu focus. All DOM work lives here.
import { MISSIONS, MISSION_IDS, UPGRADES } from './data.js';
import { formatTime, missionRecord } from './save.js';
import { t, getLang, setText } from './i18n.js';

const $ = (id) => document.getElementById(id);
const SCREENS = ['title', 'settings', 'levelup', 'pause', 'over', 'won', 'error'];

/** The values a mission's text can mention, such as its time limit. */
const missionVars = (mission) => ({ time: formatTime(mission.goal), goal: mission.goal });
const INPUT_LOCK_MS = 300;

export function createUI({ onAction }) {
  let current = null;
  let lockedUntil = 0;
  const cache = {};

  document.addEventListener('click', (e) => {
    const btn = e.target.closest('button[data-action]');
    if (!btn || performance.now() < lockedUntil) return;
    onAction(btn.dataset.action, btn.dataset.value);
  });

  function buttons() {
    return current ? Array.from($(current).querySelectorAll('button')) : [];
  }

  /** Show one screen (or none). A short lock stops a held button from picking something by accident. */
  function show(name) {
    for (const id of SCREENS) $(id).hidden = id !== name;
    current = name || null;
    if (current) {
      lockedUntil = performance.now() + INPUT_LOCK_MS;
      const first = buttons()[0];
      if (first) first.focus({ preventScroll: true });
    } else if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
  }

  /** Move focus through the open screen's buttons. Returns true if focus moved. */
  function navigate(dx, dy) {
    const list = buttons();
    if (!list.length) return false;
    const i = list.indexOf(document.activeElement);
    const step = dx > 0 || dy > 0 ? 1 : -1;
    const next = i < 0 ? 0 : (i + step + list.length) % list.length;
    list[next].focus({ preventScroll: true });
    return true;
  }

  /** Click the focused button (gamepad confirm). */
  function confirm() {
    const el = document.activeElement;
    if (current && el instanceof HTMLButtonElement && $(current).contains(el)) {
      el.click();
      return true;
    }
    return false;
  }

  function set(id, prop, value) {
    const key = id + prop;
    if (cache[key] === value) return;
    cache[key] = value;
    if (prop === 'text') $(id).textContent = value;
    else $(id).style.setProperty(prop, value);
  }

  function hud({ hp, maxHp, xp, xpNext, level, time, kills, upgrades, mission, goal, progress }) {
    set('hpFill', 'width', `${Math.max(0, Math.min(100, (hp / maxHp) * 100)).toFixed(1)}%`);
    set('hpText', 'text', String(Math.ceil(hp)));
    set('xpFill', 'width', `${Math.max(0, Math.min(100, (xp / xpNext) * 100)).toFixed(1)}%`);
    set('levelText', 'text', t('hud.level', { n: level }));
    // A timed mission counts down to its goal; endless counts up.
    const timed = mission.id === MISSION_IDS.survive;
    set('timeText', 'text', formatTime(timed ? Math.ceil(goal - progress) : time));
    set('goalText', 'text', timed ? t(`mission.${mission.name}.hud`) : '');
    set('goalText', 'display', timed ? '' : 'none');
    set('killsText', 'text', String(kills));
    const sig = getLang() + upgrades.join(',');
    if (cache.chips !== sig) {
      cache.chips = sig;
      $('chips').replaceChildren(...upgrades.flatMap((lvl, id) => {
        if (!lvl) return [];
        const chip = document.createElement('div');
        chip.className = 'chip';
        chip.title = t(`upgrade.${id}.name`);
        const glyph = document.createElement('b');
        glyph.textContent = UPGRADES[id].glyph;
        chip.append(glyph, String(lvl));
        return [chip];
      }));
    }
  }

  function showHud(visible) {
    $('hud').hidden = !visible;
  }

  function levelUp(choices, upgrades) {
    $('cards').replaceChildren(...choices.flatMap((id, slot) => {
      if (id < 0) return [];
      const u = UPGRADES[id];
      const card = document.createElement('button');
      card.className = 'card';
      card.dataset.action = 'choose';
      card.dataset.value = String(slot);
      const lvl = upgrades[id];
      const parts = [
        ['key', String(slot + 1)],
        ['glyph', u.glyph],
        ['name', t(`upgrade.${id}.name`)],
        ['rank', lvl === 0 ? t('levelup.new') : t('levelup.rank', { from: lvl, to: lvl + 1 })],
        ['desc', t(`upgrade.${id}.desc`)],
      ];
      for (const [cls, text] of parts) {
        const span = document.createElement('span');
        span.className = cls;
        span.textContent = text;
        card.append(span);
      }
      return [card];
    }));
    show('levelup');
  }

  function statRows(id, rows) {
    $(id).replaceChildren(...rows.flatMap(([label, value, best]) => {
      const dt = document.createElement('dt');
      dt.textContent = label;
      const dd = document.createElement('dd');
      dd.textContent = value;
      if (best) dd.className = 'best';
      return [dt, dd];
    }));
  }

  /** The run ended in death. `beaten` says which endless records fell; null for other missions. */
  function gameOver(mission, { time, kills, level }, save, beaten) {
    const rows = [
      [t('over.survived'), formatTime(time), beaten?.time],
      [t('over.kills'), String(kills), beaten?.kills],
      [t('over.level'), String(level), beaten?.level],
    ];
    if (beaten) rows.push([t('over.bestTime'), formatTime(save.bestTime), false]);
    else rows.unshift([t('over.mission'), t(`mission.${mission.name}.name`), false]);
    statRows('stats', rows);
    $('recordStamp').hidden = !(beaten && (beaten.time || beaten.kills || beaten.level));
    show('over');
  }

  /** The mission's goal was met. */
  function missionWon(mission, { time, kills, level }, save) {
    $('wonMission').textContent = t(`mission.${mission.name}.name`);
    statRows('wonStats', [
      [t('won.time'), formatTime(time), false],
      [t('over.kills'), String(kills), false],
      [t('over.level'), String(level), false],
      [t('won.wins'), String(missionRecord(save, mission.name).wins), false],
    ]);
    show('won');
  }

  /** The mission buttons on the title screen: goal missions first, endless last. */
  function missions(save) {
    const order = [...MISSIONS.filter((m) => m.id !== MISSION_IDS.endless), MISSIONS[MISSION_IDS.endless]];
    const focused = document.activeElement?.dataset?.value;
    $('missions').replaceChildren(...order.map((mission) => {
      const btn = document.createElement('button');
      btn.className = 'mission';
      btn.dataset.action = 'start';
      btn.dataset.value = String(mission.id);
      const name = document.createElement('span');
      name.textContent = t(`mission.${mission.name}.name`);
      const desc = document.createElement('small');
      const { wins } = missionRecord(save, mission.name);
      desc.textContent = t(`mission.${mission.name}.desc`, missionVars(mission))
        + (wins ? ` · ${t('mission.cleared', { n: wins })}` : '');
      btn.append(name, desc);
      return btn;
    }));
    // Rebuilding drops focus; put it back for keyboard and gamepad players.
    if (current === 'title') {
      const again = [...$('missions').children].find((b) => b.dataset.value === focused);
      (again || $('missions').firstElementChild).focus({ preventScroll: true });
    }
  }

  function records(save) {
    missions(save);
    $('records').textContent = save.runs
      ? t(save.runs === 1 ? 'records.one' : 'records.other', {
        time: formatTime(save.bestTime), kills: save.bestKills, level: save.bestLevel, runs: save.runs,
      })
      : '';
  }

  function settings({ volume, muted }) {
    const on = Math.round(volume * 10);
    $('volPips').replaceChildren(...Array.from({ length: 10 }, (_, i) => {
      const pip = document.createElement('i');
      if (i < on) pip.className = 'on';
      return pip;
    }));
    for (const btn of document.querySelectorAll('button[data-action="mute"]')) {
      btn.textContent = t(muted ? 'settings.soundOff' : 'settings.soundOn');
    }
    setText($('langBtn'), t('settings.language', { name: t('lang.name') }));
  }

  function error(message) {
    $('errorText').textContent = message;
    show('error');
  }

  return {
    show, navigate, confirm, hud, showHud, levelUp, gameOver, missionWon, records, settings, error,
    get screen() { return current; },
    get locked() { return performance.now() < lockedUntil; },
  };
}
