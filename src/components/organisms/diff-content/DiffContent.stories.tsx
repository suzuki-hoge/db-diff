import type { Meta, StoryObj } from '@storybook/react'

import { DiffContent } from './DiffContent'

const meta = {
  title: 'Organisms/DiffContent',
  component: DiffContent,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof DiffContent>

export default meta
type Story = StoryObj<typeof meta>

export const RowDeleted: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {},
    },
  },
}

export const RowAdded: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {},
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
      },
    },
  },
}

export const RowModified: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const RowsDeleted: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2': {
          name: { status: 'deleted', value: '"Alice"' },
          age: { status: 'deleted', value: '31' },
        },
      },
      rowDiffs2: {},
    },
  },
}

export const RowsAdded: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {},
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
        '2': {
          name: { status: 'added', value: '"Alice"' },
          age: { status: 'added', value: '31' },
        },
      },
    },
  },
}

export const RowsModified: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2': {
          name: { status: 'deleted', value: '"Alice"' },
          age: { status: 'deleted', value: '31' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
        '2': {
          name: { status: 'added', value: '"Bob"' },
          age: { status: 'added', value: '42' },
        },
      },
    },
  },
}

export const RowsModifiedAndDeleted: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2': {
          name: { status: 'deleted', value: '"Alice"' },
          age: { status: 'deleted', value: '31' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const RowsModifiedAndAdded: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1', '2'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
        '2': {
          name: { status: 'added', value: '"Alice"' },
          age: { status: 'added', value: '31' },
        },
      },
    },
  },
}

export const ColModified: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const ColRemoved: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
        },
      },
    },
  },
}

export const ColCreated: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
      },
    },
  },
}

export const ColMismatched: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1'],
      primaryColName: 'id',
      colNames: ['name', 'age', 'rate'],
      rowDiffs1: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '1': {
          name: { status: 'stay', value: '"John"' },
          rate: { status: 'added', value: '1' },
        },
      },
    },
  },
}

export const RowModifiedStringId: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['"EF974256-0BDE-4170-A2FC-4BDBBD696FB5"'],
      primaryColName: 'id',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '"EF974256-0BDE-4170-A2FC-4BDBBD696FB5"': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
      },
      rowDiffs2: {
        '"EF974256-0BDE-4170-A2FC-4BDBBD696FB5"': {
          name: { status: 'added', value: '"Jane"' },
          age: { status: 'added', value: '15' },
        },
      },
    },
  },
}

export const RowsModifiedMultiUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'users',
      primaryValues: ['1-"001"', '1-"002"', '2-"001"'],
      primaryColName: 'id-code',
      colNames: ['name', 'age'],
      rowDiffs1: {
        '1-"001"': {
          name: { status: 'deleted', value: '"John"' },
          age: { status: 'deleted', value: '29' },
        },
        '2-"001"': {
          name: { status: 'stay', value: '"Alice"' },
          age: { status: 'deleted', value: '25' },
        },
      },
      rowDiffs2: {
        '1-"002"': {
          name: { status: 'added', value: '"John"' },
          age: { status: 'added', value: '29' },
        },
        '2-"001"': {
          name: { status: 'stay', value: '"Alice"' },
          age: { status: 'added', value: '31' },
        },
      },
    },
  },
}

export const RowAddOnlyUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'relations',
      primaryValues: ['1-"001"'],
      primaryColName: 'id-code',
      colNames: [],
      rowDiffs1: {
        '1-"001"': {},
      },
      rowDiffs2: {},
    },
  },
}

export const RowDeletedOnlyUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'relations',
      primaryValues: ['1-"002"'],
      primaryColName: 'id-code',
      colNames: [],
      rowDiffs1: {},
      rowDiffs2: {
        '1-"002"': {},
      },
    },
  },
}

export const RowModifiedOnlyUniqueColumn: Story = {
  args: {
    tableDiff: {
      tableName: 'relations',
      primaryValues: ['1-"001"', '1-"002"'],
      primaryColName: 'id-code',
      colNames: [],
      rowDiffs1: {
        '1-"001"': {},
      },
      rowDiffs2: {
        '1-"002"': {},
      },
    },
  },
}
