export type LifecycleStep = {
  number: string
  command: string
  title: string
  description: string
  code: string
}

export const lifecycleSteps: LifecycleStep[] = [
  {
    number: '01',
    command: 'create',
    title: 'Create',
    description:
      'Generate a valid local skill package skeleton so the lifecycle starts and stays in one native tool.',
    code: 'skillmine create my-skill',
  },
  {
    number: '02',
    command: 'add',
    title: 'Add / Register',
    description:
      'Register the generated local skill or a remote source in declarative config as managed state.',
    code: 'skillmine add ./my-skill',
  },
  {
    number: '03',
    command: 'install',
    title: 'Install',
    description:
      'Resolve and cache skills in content-addressable storage. Deterministic, reproducible, and reviewable.',
    code: 'skillmine install',
  },
  {
    number: '04',
    command: 'sync',
    title: 'Sync',
    description:
      'Expose skills to your assistant runtime. Works with Claude Code, OpenCode, and Cursor.',
    code: 'skillmine sync --target=claude',
  },
  {
    number: '05',
    command: 'doctor',
    title: 'Doctor',
    description:
      'Run diagnostics to check configuration, detect drift, and validate the whole loop end to end.',
    code: 'skillmine doctor',
  },
]
