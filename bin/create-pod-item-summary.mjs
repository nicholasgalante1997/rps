#! /usr/bin/env node

import fsp from 'node:fs/promises'
import path from 'node:path';
import os from 'node:os';

const PODS_ROOT_PATH = path.resolve(process.cwd(), '.pods');
const PODS_RESOURCE_LIST_PATH = path.resolve(PODS_ROOT_PATH, 'resources.json');
const data = {
    '$schema': 'TODO',
    timestamp: new Date().toUTCString(),
    resources: await flatten(await getDirEnts(PODS_ROOT_PATH))
};

await fsp.rm(PODS_RESOURCE_LIST_PATH, { force: true, recursive: true, maxRetries: 3 });
await fsp.writeFile(
    PODS_RESOURCE_LIST_PATH, 
    JSON.stringify(data),
    {
        encoding: 'utf-8',
    }
);

async function getDirEnts(dirname) {
    return await fsp.readdir(dirname, { encoding: 'utf8', recursive: true, withFileTypes: true })
}

async function flatten(dirEnts, tracker = new Set()) {
    for (const dirEnt of dirEnts) {
        if (dirEnt.isFile()) {
            tracker.add(getNormalizedPath(dirEnt.parentPath), dirEnt.name);
        } else if (dirEnt.isDirectory()) {
            flatten(await getDirEnts(path.resolve(dirEnt.parentPath, dirEnt.name)), tracker);
        }
    }
    return Array.from(tracker);
}

function getNormalizedPath(parentPath) {
    const delimiter = os.platform() === 'win32' ? '\\' : '/';
    const paths = parentPath.split(delimiter);
    const start = paths.indexOf('.pods');
    return paths.slice(start).join(delimiter);
}