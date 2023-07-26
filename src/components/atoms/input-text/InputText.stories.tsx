import type { Meta, StoryObj } from '@storybook/react'

import { InputText } from './InputText'

const meta = {
  title: 'Atoms/InputText',
  component: InputText,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof InputText>

export default meta
type Story = StoryObj<typeof meta>

export const Empty: Story = {
  args: {
    value: '',
    length: 10,
  },
}

export const HalfWithMax: Story = {
  args: {
    value: '0123456789',
    length: 10,
  },
}

export const FullWithMax: Story = {
  args: {
    value: 'あいうえおかきくけこ',
    length: 10,
  },
}
