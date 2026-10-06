import json
from pathlib import Path

SOURCE = Path('/home/user/output/canvas/dashboard.json')
TARGET = Path('/home/user/output/canvas/CANVAS.md')


def esc(value):
    return (str(value).replace('&', '&amp;').replace('<', '&lt;')
            .replace('>', '&gt;').replace('"', '&quot;').replace("'", '&#x27;'))


def check(condition, message):
    if not condition:
        raise ValueError(message)


def number(value):
    check(isinstance(value, (int, float)) and not isinstance(value, bool), 'chart values must be numeric')
    return float(value)


def chart_data(widget):
    categories = widget.get('categories')
    series = widget.get('series')
    check(isinstance(categories, list) and categories, 'chart categories are required')
    check(isinstance(series, list) and series, 'chart series are required')
    result = []
    for item in series:
        values = item.get('values') if isinstance(item, dict) else None
        check(isinstance(values, list) and len(values) == len(categories), 'series values must match categories')
        result.append((esc(item.get('name', 'Series')), [number(value) for value in values]))
    return [esc(value) for value in categories], result


def metric(widget):
    result = '<div class="metric-value">' + esc(widget.get('value', '')) + '</div>'
    if 'change' in widget:
        trend = widget.get('trend', 'neutral')
        check(trend in ('up', 'down', 'neutral'), 'invalid metric trend')
        result += '<div class="change ' + trend + '">' + esc(widget['change']) + '</div>'
    return result


def bar(widget):
    categories, series = chart_data(widget)
    maximum = max([abs(value) for _, values in series for value in values] or [1])
    rows = []
    for index, category in enumerate(categories):
        bars = []
        for name, values in series:
            width = abs(values[index]) / maximum * 100
            bars.append('<div class="bar-row"><span>' + name + '</span><div class="bar-track"><i style="width:' + str(width) + '%"></i></div><b>' + esc(values[index]) + '</b></div>')
        rows.append('<div class="category"><strong>' + category + '</strong>' + ''.join(bars) + '</div>')
    return '<div class="bars">' + ''.join(rows) + '</div>'


def line(widget):
    categories, series = chart_data(widget)
    values = [value for _, data in series for value in data]
    low = min(values)
    span = max(values) - low or 1
    lines = []
    colors = ['#2563eb', '#0f766e', '#c2410c', '#7c3aed']
    for series_index, (name, data) in enumerate(series):
        points = []
        for index, value in enumerate(data):
            x = 20 + index * (360 / max(1, len(categories) - 1))
            y = 170 - ((value - low) / span * 140)
            points.append(str(round(x, 1)) + ',' + str(round(y, 1)))
        color = colors[series_index % len(colors)]
        lines.append('<polyline points="' + ' '.join(points) + '" fill="none" stroke="' + color + '" stroke-width="3"/><text x="' + str(20 + series_index * 90) + '" y="195" fill="' + color + '">' + name + '</text>')
    labels = ''.join('<text x="' + str(20 + index * (360 / max(1, len(categories) - 1))) + '" y="218" text-anchor="middle">' + category + '</text>' for index, category in enumerate(categories))
    return '<svg viewBox="0 0 400 230" role="img" aria-label="Line chart">' + ''.join(lines) + labels + '</svg>'


def pie(widget):
    labels = widget.get('labels')
    values = widget.get('values')
    check(isinstance(labels, list) and isinstance(values, list) and labels and len(labels) == len(values), 'pie labels and values must match')
    check(len(labels) <= 6, 'pie charts cannot contain more than six categories')
    values = [number(value) for value in values]
    total = sum(values)
    check(total > 0, 'pie values must have a positive total')
    colors = ['#2563eb', '#0f766e', '#c2410c', '#7c3aed', '#ca8a04', '#be123c']
    cursor = 0
    stops = []
    legend = []
    for index, value in enumerate(values):
        end = cursor + value / total * 100
        stops.append(colors[index] + ' ' + str(cursor) + '% ' + str(end) + '%')
        legend.append('<li><i style="background:' + colors[index] + '"></i>' + esc(labels[index]) + ' <b>' + esc(value) + '</b></li>')
        cursor = end
    return '<div class="pie-layout"><div class="pie" style="background:conic-gradient(' + ','.join(stops) + ')"></div><ul class="legend">' + ''.join(legend) + '</ul></div>'


def table(widget):
    columns = widget.get('columns')
    rows = widget.get('rows')
    check(isinstance(columns, list) and isinstance(rows, list), 'table columns and rows are required')
    check(all(isinstance(row, list) and len(row) == len(columns) for row in rows), 'table rows must match columns')
    head = ''.join('<th>' + esc(column) + '</th>' for column in columns)
    body = ''.join('<tr>' + ''.join('<td>' + esc(value) + '</td>' for value in row) + '</tr>' for row in rows)
    return '<div class="table-wrap"><table><thead><tr>' + head + '</tr></thead><tbody>' + body + '</tbody></table></div>'


def alert(widget):
    severity = widget.get('severity', 'info')
    check(severity in ('info', 'warning', 'critical', 'success'), 'invalid alert severity')
    return '<div class="alert ' + severity + '"><strong>' + esc(severity.upper()) + '</strong><p>' + esc(widget.get('text', '')) + '</p></div>'


def widget_html(widget):
    check(isinstance(widget, dict), 'each widget must be an object')
    kind = widget.get('type')
    renderers = {'metric': metric, 'bar': bar, 'line': line, 'pie': pie, 'table': table, 'alert': alert}
    check(kind in renderers, 'unsupported widget type: ' + str(kind))
    return '<section class="widget ' + esc(kind) + '"><h2>' + esc(widget.get('title', kind.title())) + '</h2>' + renderers[kind](widget) + '</section>'


data = json.loads(SOURCE.read_text())
check(isinstance(data, dict), 'dashboard must be an object')
widgets = data.get('widgets')
check(isinstance(widgets, list) and 1 <= len(widgets) <= 7, 'dashboard must contain between one and seven widgets')
title = esc(data.get('title', 'Dashboard'))
subtitle = esc(data.get('subtitle', ''))
body = ''.join(widget_html(widget) for widget in widgets)
css = '*{box-sizing:border-box}body{margin:0;background:#f8fafc;color:#172033;font:15px/1.5 system-ui,sans-serif}.dashboard{max-width:1100px;margin:auto;padding:28px}.header{margin-bottom:22px}.header h1{margin:0 0 4px;font-size:28px}.header p{margin:0;color:#64748b}.grid{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:16px}.widget{background:white;border:1px solid #dbe2ea;border-radius:10px;padding:18px;box-shadow:0 2px 8px #1720330d}.widget h2{font-size:15px;margin:0 0 16px}.metric-value{font-size:36px;font-weight:700}.change{margin-top:8px;font-weight:600}.up,.success{color:#047857}.down,.critical{color:#be123c}.neutral,.info{color:#475569}.warning{color:#b45309}.bars{display:grid;gap:14px}.category>strong{display:block;margin-bottom:5px}.bar-row{display:grid;grid-template-columns:8rem 1fr auto;gap:8px;align-items:center;font-size:12px}.bar-track{height:9px;background:#e2e8f0;border-radius:99px;overflow:hidden}.bar-track i{display:block;height:100%;background:#2563eb;border-radius:99px}.widget svg{width:100%;height:auto}.widget svg text{font-size:10px}.pie-layout{display:flex;align-items:center;gap:24px}.pie{width:150px;height:150px;border-radius:50%}.legend{list-style:none;padding:0;margin:0;display:grid;gap:7px}.legend li{display:flex;gap:7px;align-items:center}.legend i{width:11px;height:11px;border-radius:50%}.table-wrap{overflow:auto}table{width:100%;border-collapse:collapse}th,td{text-align:left;padding:9px;border-bottom:1px solid #e2e8f0;white-space:nowrap}th{font-size:12px;color:#64748b}.alert{border-left:4px solid currentColor;background:#f8fafc;padding:12px 14px}.alert p{margin:4px 0 0;color:#334155}.table,.alert{grid-column:span 2}@media(max-width:700px){.dashboard{padding:16px}.grid{grid-template-columns:1fr}.table,.alert{grid-column:span 1}.pie-layout{align-items:flex-start;flex-direction:column}}'
document = '---\nname: dashboard\ntitle: ' + title + '\ntype: text/html\n---\n<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>' + title + '</title><style>' + css + '</style></head><body><main class="dashboard"><header class="header"><h1>' + title + '</h1><p>' + subtitle + '</p></header><div class="grid">' + body + '</div></main></body></html>'
TARGET.parent.mkdir(parents=True, exist_ok=True)
TARGET.write_text(document)
print(str(TARGET))
